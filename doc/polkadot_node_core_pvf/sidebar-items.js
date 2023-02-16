window.SIDEBAR_ITEMS = {"constant":[["JOB_TIMEOUT_WALL_CLOCK_FACTOR","A multiple of the job timeout (in CPU time) for which we are willing to wait on the host (in wall clock time). This is lenient because CPU time may go slower than wall clock time."]],"enum":[["InvalidCandidate","A description of an error raised during executing a PVF and can be attributed to the combination of the candidate [`polkadot_parachain::primitives::ValidationParams`] and the PVF."],["PrepareError","An error that occurred during the prepare part of the PVF pipeline."],["Priority","A priority assigned to execution of a PVF."],["ValidationError","A error raised during validation of the candidate."]],"fn":[["execute_worker_entrypoint","The entrypoint that the spawned execute worker should start with. The `socket_path` specifies the path to the socket used to communicate with the host."],["prepare","Runs preparation on the given runtime blob. If successful, it returns a serialized compiled artifact which can then be used to pass into `Executor::execute` after writing it to the disk."],["prepare_worker_entrypoint","The entrypoint that the spawned prepare worker should start with. The `socket_path` specifies the path to the socket used to communicate with the host."],["prevalidate","Runs the prevalidation on the given code. Returns a [`RuntimeBlob`] if it succeeds."],["start","Start the validation host."]],"macro":[["decl_puppet_worker_main","Use this macro to declare a `fn main() {}` that will check the arguments and dispatch them to the appropriate worker, making the executable that can be used for spawning workers."]],"struct":[["Config","Configuration for the validation host."],["Metrics","Validation host metrics."],["PrepareStats","Preparation statistics, including the CPU time and memory taken."],["Pvf","A struct that carries code of a parachain validation function and its hash."],["PvfWithExecutorParams","Coupling PVF code with executor params"],["ValidationHost","A handle to the async process serving the validation host requests."]],"type":[["PrepareResult","Result of PVF preparation performed by the validation host. Contains stats about the preparation if successful"]]};