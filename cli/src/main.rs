#[macro_use]
extern crate lazy_static;
extern crate ansi_term;
extern crate clap_verbosity_flag;
extern crate loggerv;
use anyhow::{Context, Result};

use log::debug;
use {{crate_name}}_lib::hello;

use structopt::StructOpt;

fn main() -> Result<()> {
    lazy_static! {
        static ref OPT: Opt = Opt::from_args();
    }

    #[cfg(windows)]
    ansi_term::enable_ansi_support().unwrap();

    loggerv::Logger::new()
        .max_level(
            OPT.verbosity
                .log_level()
                .context("Failed to get log level")?,
        )
        .level(OPT.debug)
        .module_path(OPT.debug)
        .line_numbers(OPT.debug)
        .init()
        .unwrap();

    debug!("{:#?}", *OPT);

    hello(&OPT.name.as_str()).context("Failed to say hello")?;

    Ok(())
}

#[derive(Debug, StructOpt)]
#[structopt(
    name = "{{project-name}}",
    about = "{{description}}"
)]
struct Opt {
    #[structopt(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,

    /// Name to greet.
    #[structopt(name = "NAME")]
    name: String,

    /// Enables debug mode
    #[structopt(short, long)]
    debug: bool,
}
