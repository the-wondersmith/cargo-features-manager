use std::io::stdout;

use clap::{arg, Parser};
use crossterm::execute;
use crossterm::style::{Print, Stylize};

use crate::display::Display;

mod crates;
mod display;
mod document;
mod index;

#[derive(Parser)] // requires `derive` feature
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
enum CargoCli {
    Features(FeaturesArgs),
}

#[derive(clap::Args)]
#[command(author, version, about, long_about = None)]
struct FeaturesArgs {
    #[arg(long, short)]
    dependency: Option<String>,
}

fn main() {
    let CargoCli::Features(args) = CargoCli::parse();

    if let Err(err) = run(args) {
        execute!(
            stdout(),
            Print("error".red().bold()),
            Print(": "),
            Print(err.to_string())
        )
        .unwrap();
    }
}

fn run(args: FeaturesArgs) -> anyhow::Result<()> {
    let mut display = Display::new()?;

    if let Some(name) = args.dependency {
        display.set_selected_dep(name)?
    }

    display.start()?;

    Ok(())
}
