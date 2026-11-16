use clap::Parser;
use hbb_common::crypt;

#[derive(Parser, Debug)]
pub struct Opts {
    /// String to hash
    #[clap(short, long)]
    string: String,
}

pub fn string_hash(opts: Opts) -> anyhow::Result<()> {
    println!("Hash of {}: {}", opts.string, crypt::legacy_password_hash(&opts.string));
    Ok(())
}
