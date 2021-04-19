use assert_cmd::cargo::cargo_bin;
use std::{convert::TryInto, fs, process::Command, thread, time::Duration};

mod common;

#[test]
#[cfg(unix)]
fn running_the_node_works_and_can_be_interrupted() {
	use nix::{
		sys::signal::{
			kill,
			Signal::{self, SIGINT, SIGTERM},
		},
		unistd::Pid,
	};

	fn run_command_and_kill(signal: Signal) {
		let _ = fs::remove_dir_all("interrupt_test");
		let mut cmd = Command::new(cargo_bin("rococo-collator"))
			.args(&["-d", "interrupt_test", "--", "--dev"])
			.spawn()
			.unwrap();

		thread::sleep(Duration::from_secs(30));
		assert!(
			cmd.try_wait().unwrap().is_none(),
			"the process should still be running"
		);
		kill(Pid::from_raw(cmd.id().try_into().unwrap()), signal).unwrap();
		assert_eq!(
			common::wait_for(&mut cmd, 30).map(|x| x.success()),
			Some(true),
			"the process must exit gracefully after signal {}",
			signal,
		);
	}

	run_command_and_kill(SIGINT);
	run_command_and_kill(SIGTERM);
}
