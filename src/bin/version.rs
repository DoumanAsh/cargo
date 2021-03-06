use std::env;

use cargo;
use cargo::util::{CliResult, Config};

#[derive(RustcDecodable)]
pub struct Options;

pub const USAGE: &'static str = "
Show version information

Usage:
    cargo version [options]

Options:
    -h, --help               Print this message
    -v, --verbose            Use verbose output
    --color WHEN             Coloring: auto, always, never
";

pub fn execute(_: Options, _: &Config) -> CliResult<Option<()>> {
    debug!("executing; cmd=cargo-version; args={:?}", env::args().collect::<Vec<_>>());

    println!("{}", cargo::version());

    Ok(None)
}
