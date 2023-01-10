extern crate ansi_term;
extern crate clap_verbosity_flag;
extern crate loggerv;
use anyhow::{Context, Result};

use log::{debug, info};
use {{crate_name}}_lib_a::hello;
use {{crate_name}}_lib_b::add;

use clap::Parser;

fn main() -> Result<()> {
    let args = Args::parse();

    setup(&args).context("Failed to setup application environment")?;

    hello(args.name.as_str()).context("Failed to say hello")?;

    info!("1 + 1 = {}", add(1, 1).context("Failed to add 1 and 1")?);

    Ok(())
}

fn setup(opt: &Args) -> Result<()> {
    #[cfg(windows)]
    ansi_term::enable_ansi_support().context("Failed to enable ansi support")?;

    loggerv::Logger::new()
        .max_level(
            opt.verbosity
                .log_level()
                .context("Failed to get log level")?,
        )
        .level(opt.debug)
        .module_path(opt.debug)
        .line_numbers(opt.debug)
        .init()
        .context("Failed to setup logger")?;

    debug!("{:#?}", *opt);

    Ok(())
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, name = "{{project-name}}", about = "{{description}}")]
struct Args {
    #[structopt(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,

    /// Name to greet.
    #[structopt(name = "NAME")]
    name: String,

    /// Enables debug mode
    #[structopt(short, long)]
    debug: bool,
}
