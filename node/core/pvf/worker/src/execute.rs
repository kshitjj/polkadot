// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

use crate::{
	common::{
		bytes_to_path, cond_notify_on_done, cond_wait_while, cpu_time_monitor_loop,
		stringify_panic_payload, worker_event_loop, WaitOutcome,
	},
	executor_intf::{Executor, EXECUTE_THREAD_STACK_SIZE},
	LOG_TARGET,
};
use cpu_time::ProcessTime;
use parity_scale_codec::{Decode, Encode};
use polkadot_node_core_pvf::{
	framed_recv, framed_send, ExecuteHandshake as Handshake, ExecuteResponse as Response,
};
use polkadot_parachain::primitives::ValidationResult;
use std::{
	path::{Path, PathBuf},
	sync::{mpsc::channel, Arc, Condvar, Mutex},
	thread,
	time::Duration,
};
use tokio::{io, net::UnixStream};

async fn recv_handshake(stream: &mut UnixStream) -> io::Result<Handshake> {
	let handshake_enc = framed_recv(stream).await?;
	let handshake = Handshake::decode(&mut &handshake_enc[..]).map_err(|_| {
		io::Error::new(
			io::ErrorKind::Other,
			"execute pvf recv_handshake: failed to decode Handshake".to_owned(),
		)
	})?;
	Ok(handshake)
}

async fn recv_request(stream: &mut UnixStream) -> io::Result<(PathBuf, Vec<u8>, Duration)> {
	let artifact_path = framed_recv(stream).await?;
	let artifact_path = bytes_to_path(&artifact_path).ok_or_else(|| {
		io::Error::new(
			io::ErrorKind::Other,
			"execute pvf recv_request: non utf-8 artifact path".to_string(),
		)
	})?;
	let params = framed_recv(stream).await?;
	let execution_timeout = framed_recv(stream).await?;
	let execution_timeout = Duration::decode(&mut &execution_timeout[..]).map_err(|_| {
		io::Error::new(
			io::ErrorKind::Other,
			"execute pvf recv_request: failed to decode duration".to_string(),
		)
	})?;
	Ok((artifact_path, params, execution_timeout))
}

async fn send_response(stream: &mut UnixStream, response: Response) -> io::Result<()> {
	framed_send(stream, &response.encode()).await
}

/// The entrypoint that the spawned execute worker should start with.
///
/// # Parameters
///
/// The `socket_path` specifies the path to the socket used to communicate with the host. The
/// `node_version`, if `Some`, is checked against the worker version. A mismatch results in
/// immediate worker termination. `None` is used for tests and in other situations when version
/// check is not necessary.
pub fn worker_entrypoint(socket_path: &str, node_version: Option<&str>) {
	worker_event_loop("execute", socket_path, node_version, |mut stream| async move {
		let worker_pid = std::process::id();

		let handshake = recv_handshake(&mut stream).await?;
		let executor = Executor::new(handshake.executor_params).map_err(|e| {
			io::Error::new(io::ErrorKind::Other, format!("cannot create executor: {}", e))
		})?;

		loop {
			let (artifact_path, params, execution_timeout) = recv_request(&mut stream).await?;
			gum::debug!(
				target: LOG_TARGET,
				%worker_pid,
				"worker: validating artifact {}",
				artifact_path.display(),
			);

			// Conditional variable to notify us when a thread is done.
			let cond_main = Arc::new((Mutex::new(WaitOutcome::Pending), Condvar::new()));
			let cond_cpu = Arc::clone(&cond_main);
			let cond_job = Arc::clone(&cond_main);

			let cpu_time_start = ProcessTime::now();

			// Spawn a new thread that runs the CPU time monitor.
			let (cpu_time_monitor_tx, cpu_time_monitor_rx) = channel::<()>();
			let cpu_time_monitor_thread = thread::spawn(move || {
				cond_notify_on_done(
					|| {
						cpu_time_monitor_loop(
							cpu_time_start,
							execution_timeout,
							cpu_time_monitor_rx,
						)
					},
					cond_cpu,
					WaitOutcome::CpuTimedOut,
				)
			});
			let executor_2 = executor.clone();
			let execute_thread =
				thread::Builder::new().stack_size(EXECUTE_THREAD_STACK_SIZE).spawn(move || {
					cond_notify_on_done(
						|| {
							validate_using_artifact(
								&artifact_path,
								&params,
								executor_2,
								cpu_time_start,
							)
						},
						cond_job,
						WaitOutcome::JobFinished,
					)
				})?;

			// Wait for one of the threads to finish.
			let outcome = cond_wait_while(cond_main);

			let response = match outcome {
				WaitOutcome::JobFinished => {
					let _ = cpu_time_monitor_tx.send(());
					execute_thread.join().unwrap_or_else(|e| {
						// TODO: Use `Panic` error once that is implemented.
						Response::format_internal(
							"execute thread error",
							&stringify_panic_payload(e),
						)
					})
				},
				// If this thread is not selected, we signal it to end, the join handle is dropped
				// and the thread will finish in the background.
				WaitOutcome::CpuTimedOut => {
					match cpu_time_monitor_thread.join() {
						Ok(Some(cpu_time_elapsed)) => {
							// Log if we exceed the timeout and the other thread hasn't finished.
							gum::warn!(
								target: LOG_TARGET,
								%worker_pid,
								"execute job took {}ms cpu time, exceeded execute timeout {}ms",
								cpu_time_elapsed.as_millis(),
								execution_timeout.as_millis(),
							);
							Response::TimedOut
						},
						Ok(None) => Response::format_internal(
							"cpu time monitor thread error",
							"error communicating over finished channel".into(),
						),
						// We can use an internal error here because errors in this thread are
						// independent of the candidate.
						Err(e) => Response::format_internal(
							"cpu time monitor thread error",
							&stringify_panic_payload(e),
						),
					}
				},
				WaitOutcome::Pending => Response::InternalError(
					"we run wait_while until the outcome is no longer pending; qed".into(),
				),
			};

			send_response(&mut stream, response).await?;
		}
	});
}

fn validate_using_artifact(
	artifact_path: &Path,
	params: &[u8],
	executor: Executor,
	cpu_time_start: ProcessTime,
) -> Response {
	// Check here if the file exists, because the error from Substrate is not match-able.
	// TODO: Re-evaluate after <https://github.com/paritytech/substrate/issues/13860>.
	let file_metadata = std::fs::metadata(artifact_path);
	if let Err(err) = file_metadata {
		return Response::format_internal("execute: could not find or open file", &err.to_string())
	}

	let descriptor_bytes = match unsafe {
		// SAFETY: this should be safe since the compiled artifact passed here comes from the
		//         file created by the prepare workers. These files are obtained by calling
		//         [`executor_intf::prepare`].
		executor.execute(artifact_path.as_ref(), params)
	} {
		Err(err) => return Response::format_invalid("execute", &err),
		Ok(d) => d,
	};

	let result_descriptor = match ValidationResult::decode(&mut &descriptor_bytes[..]) {
		Err(err) =>
			return Response::format_invalid("validation result decoding failed", &err.to_string()),
		Ok(r) => r,
	};

	// Include the decoding in the measured time, to prevent any potential attacks exploiting some
	// bug in decoding.
	let duration = cpu_time_start.elapsed();

	Response::Ok { result_descriptor, duration }
}
