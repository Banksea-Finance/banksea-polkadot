use std::{
	process::{Child, ExitStatus},
	thread,
	time::Duration,
};

/// Wait for the given `child` the given ammount of `secs`.
///
/// Returns the `Some(exit status)` or `None` if the process did not finish in the given time.
pub fn wait_for(child: &mut Child, secs: usize) -> Option<ExitStatus> {
	for _ in 0..secs {
		match child.try_wait().unwrap() {
			Some(status) => return Some(status),
			None => thread::sleep(Duration::from_secs(1)),
		}
	}
	eprintln!("Took to long to exit. Killing...");
	let _ = child.kill();
	child.wait().unwrap();

	None
}
