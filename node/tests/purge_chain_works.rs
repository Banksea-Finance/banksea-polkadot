use assert_cmd::cargo::cargo_bin;
use std::{convert::TryInto, process::Command, thread, time::Duration};

mod common;

#[test]
#[cfg(unix)]
fn purge_chain_works() {
	fn run_node_and_stop() -> tempfile::TempDir {
		use nix::{
			sys::signal::{kill, Signal::SIGINT},
			unistd::Pid,
		};

		let base_path = tempfile::tempdir().unwrap();

		let mut cmd = Command::new(cargo_bin("rococo-collator"))
			.args(&["-d"])
			.arg(base_path.path())
			.args(&["--"])
			.spawn()
			.unwrap();

		// Let it produce some blocks.
		thread::sleep(Duration::from_secs(30));
		assert!(
			cmd.try_wait().unwrap().is_none(),
			"the process should still be running"
		);

		// Stop the process
		kill(Pid::from_raw(cmd.id().try_into().unwrap()), SIGINT).unwrap();
		assert!(common::wait_for(&mut cmd, 30)
			.map(|x| x.success())
			.unwrap_or_default());

		base_path
	}

	// Check that both databases are deleted
	{
		let base_path = run_node_and_stop();

		assert!(base_path.path().join("chains/local_testnet/db").exists());
		assert!(base_path.path().join("polkadot/chains/westend_dev/db").exists());

		let status = Command::new(cargo_bin("rococo-collator"))
			.args(&["purge-chain", "-d"])
			.arg(base_path.path())
			.arg("-y")
			.status()
			.unwrap();
		assert!(status.success());

		// Make sure that the `parachain_local_testnet` chain folder exists, but the `db` is deleted.
		assert!(base_path.path().join("chains/local_testnet").exists());
		assert!(!base_path.path().join("chains/local_testnet/db").exists());
		assert!(base_path.path().join("polkadot/chains/westend_dev").exists());
		assert!(!base_path.path().join("polkadot/chains/westend_dev/db").exists());
	}
}
