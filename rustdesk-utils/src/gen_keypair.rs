use clap::Parser;
use hbb_common::config::Config;

#[derive(Parser, Debug)]
pub struct Opts {
    /// Output path
    #[clap(default_value = ".", short, long)]
    path: String,
}

pub fn gen_keypair(_opts: Opts) -> anyhow::Result<()> {
    let (sk, pk) = Config::get_key_pair();
    println!("Public key: {}", base64::encode(&pk));
    println!("Private key: {}", base64::encode(&sk));
    Ok(())
}
