use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Raine")]
struct Opts {
    #[clap(subcommand)]
    subcmd: MainCommand,
}

#[derive(Parser)]
enum MainCommand {
    Init,       // Creates the project, does support --lang arg
    Eject,      // Removes everything from the project, this is final.
    
}

fn main() -> anyhow::Result<()> {
    println!("Initializing project template");
    let opts: Opts = Opts::parse();

    opts.subcmd.execute_init_command()?;
    Ok(())
}
