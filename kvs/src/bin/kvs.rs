use std::path::PathBuf;
use structopt::{clap, StructOpt};

#[derive(StructOpt, Debug)]
#[structopt(
    name = "kvs",
    about = "kvs cli",
    version(env!("CARGO_PKG_VERSION")),
    setting(clap::AppSettings::ArgRequiredElseHelp),
    global_settings(&[
        clap::AppSettings::ColoredHelp,
    ]),
)]
pub struct Opt {
    #[structopt(
        long = "file",
        short = "f",
        global = true,
        help = "specify log data file.",
        env = "KVS_FILE",
        default_value = ".data.kvs"
    )]
    pub file: PathBuf,
}

fn run() -> Result<(), anyhow::Error> {
    let opt = Opt::from_args();

    println!("file {:?}", opt.file);

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{}", err);
        std::process::exit(1);
    }
}
