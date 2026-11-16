use clap::Parser;
use anyhow::Result;

mod gen_keypair;
mod import_keypair;
mod string_hash;

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(subcommand)]
    cmd: SubCmd,
}

#[derive(Parser, Debug)]
enum SubCmd {
    GenKeypair(gen_keypair::Opts),
    ImportKeypair(import_keypair::Opts),
    StringHash(string_hash::Opts),
}

fn main() -> Result<()> {
    let args = Args::parse();
    match args.cmd {
        SubCmd::GenKeypair(opts) => gen_keypair::gen_keypair(opts),
        SubCmd::ImportKeypair(opts) => import_keypair::import_keypair(opts),
        SubCmd::StringHash(opts) => string_hash::string_hash(opts),
    }
}
