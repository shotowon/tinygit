use crate::cli_parser::{Query, Commands};

use std::error::Error;

mod lib;
mod common;
mod consts;
mod object;

pub fn run(query: Query) -> Result<(), Box<dyn Error>> {
    match query.command {
        Commands::CatFile { pretty, object } => lib::cat_file(pretty, object)?,
        Commands::Init => lib::init()?,
    }

    Ok(())
}
