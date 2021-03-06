use kvs::{cli, Kvs, KvsError};
use std::path::PathBuf;
use structopt::{clap, StructOpt};

#[derive(StructOpt, Debug)]
#[structopt(
    name = "kvs",
    about = "kvs cli",
    version(env ! ("CARGO_PKG_VERSION")),
    setting(clap::AppSettings::ArgRequiredElseHelp),
    global_settings(& [
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
    pub file: PathBuf, // server commandでは無効にしたい

    #[structopt(subcommand)]
    pub cmd: SubCommand,
}

#[derive(StructOpt, Debug)]
pub enum SubCommand {
    #[structopt(about = "Put key value onto disk.")]
    Put {
        #[structopt(help = "key")]
        key: String,
        #[structopt(help = "value")]
        value: String,
    },
    #[structopt(about = "Get value from disk.")]
    Get {
        #[structopt(help = "key")]
        key: String,
    },
    #[structopt(about = "Mark delete to given key value. return value if key exists.")]
    Delete {
        #[structopt(help = "key")]
        key: String,
    },

    #[structopt(about = "Server mode.")]
    Server {
        #[structopt(
            long = "addr",
            help = "tcp bind address.",
            env = "KVS_ADDR",
            default_value = "0.0.0.0:4002"
        )]
        addr: String,
    },

    #[structopt(about = "Client mode.")]
    Client {
        #[structopt(
            long = "addr",
            help = "tcp bind address.",
            env = "KVS_ADDR",
            default_value = "0.0.0.0:4002"
        )]
        addr: String,
    },
}

fn run() -> Result<(), anyhow::Error> {
    let opt = Opt::from_args();

    let mut kvs = Kvs::new(opt.file)?;

    match opt.cmd {
        SubCommand::Put { key, value } => {
            kvs.put::<_, String>(key, &value)?;
        }
        SubCommand::Get { key } => {
            println!("{}", kvs.get::<String>(key.as_str())?);
        }
        SubCommand::Delete { key } => {
            if let Some(value) = kvs.delete::<String>(&key)? {
                println!("{}", value);
            }
            println!("Successfully deleted");
        }
        SubCommand::Server { addr, .. } => cli::server::server_main(addr)?,
        SubCommand::Client { addr, .. } => cli::client::client_main(addr)?,
    }

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        let code = match err.downcast_ref::<KvsError>() {
            Some(KvsError::NotFound) => {
                eprintln!("Not Found");
                2
            }
            _ => {
                eprintln!("{}", err);
                1
            }
        };
        std::process::exit(code);
    }
}
