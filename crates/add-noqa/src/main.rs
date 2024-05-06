mod add_noqa;

use crate::add_noqa::{apply_ignores};
use pyright::PyrightOutput;
use std::fs::File;
use std::path::PathBuf;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[clap(long, action)]
    inline: bool,
    #[arg()]
    pyright_output: PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let res: PyrightOutput = serde_json::from_reader(
        File::open(args.pyright_output)
            .unwrap(),
    )
        .unwrap();
    apply_ignores(res.general_diagnostics.iter(), args.inline)?;
    Ok(())
}
