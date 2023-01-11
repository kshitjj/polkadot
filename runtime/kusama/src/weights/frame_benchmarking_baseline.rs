// Copyright 2017-2022 Parity Technologies (UK) Ltd.
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
//! Autogenerated weights for `frame_benchmarking::baseline`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-08, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `bm4`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kusama-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=kusama-dev
// --steps=50
// --repeat=20
// --pallet=frame_benchmarking::baseline
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/kusama/src/weights/frame_benchmarking_baseline.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight}};
use sp_std::marker::PhantomData;

/// Weight functions for `frame_benchmarking::baseline`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> frame_benchmarking::baseline::WeightInfo for WeightInfo<T> {
	/// The range of component `i` is `[0, 1000000]`.
	fn addition(_i: u32, ) -> Weight {
		Weight::from_ref_time(114_000 as u64)
	}
	/// The range of component `i` is `[0, 1000000]`.
	fn subtraction(_i: u32, ) -> Weight {
		Weight::from_ref_time(125_000 as u64)
	}
	/// The range of component `i` is `[0, 1000000]`.
	fn multiplication(_i: u32, ) -> Weight {
		Weight::from_ref_time(116_000 as u64)
	}
	/// The range of component `i` is `[0, 1000000]`.
	fn division(_i: u32, ) -> Weight {
		Weight::from_ref_time(115_000 as u64)
	}
	/// The range of component `i` is `[0, 100]`.
	fn hashing(i: u32, ) -> Weight {
		Weight::from_ref_time(19_441_790_000 as u64)
			// Standard Error: 126_000
			.saturating_add(Weight::from_ref_time(115_000 as u64).saturating_mul(i as u64))
	}
	/// The range of component `i` is `[1, 100]`.
	fn sr25519_verification(i: u32, ) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 35_000
			.saturating_add(Weight::from_ref_time(47_909_000 as u64).saturating_mul(i as u64))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	/// The range of component `i` is `[0, 1000]`.
	fn storage_read(i: u32, ) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 7_000
			.saturating_add(Weight::from_ref_time(1_998_000 as u64).saturating_mul(i as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(i as u64)))
	}
	// Storage: Skipped Metadata (r:0 w:0)
	/// The range of component `i` is `[0, 1000]`.
	fn storage_write(i: u32, ) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(338_000 as u64).saturating_mul(i as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(i as u64)))
	}
}
