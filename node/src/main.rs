//! test parachain collator

#![warn(missing_docs)]
#![warn(unused_extern_crates)]

mod banksy_chain_spec;
#[macro_use]
mod service;
mod service_dev;
mod cli;
mod command;

fn main() -> sc_cli::Result<()> {
	command::run()
}
