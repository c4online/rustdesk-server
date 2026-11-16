use clap::Parser;

#[derive(Parser, Debug)]
pub struct Opts {
    /// Private key file path
    #[clap(short, long)]
    key: String,
    /// Output path
    #[clap(default_value = ".", short, long)]
    path: String,
}

pub fn import_keypair(_opts: Opts) -> anyhow::Result<()> {
    println!("Import keypair feature");
    Ok(())
}
