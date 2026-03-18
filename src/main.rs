mod cli;
mod log;
mod core;

use clap::Parser;
use cli::args::Args;

use log::log::{ Log, LogF };

use cli::matches::match_cli;

use crate::core::{filemanager::Fiman, project::{GraveyardList, ProjectList}};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // file manager
    let mut fileman: Fiman = Fiman::new()?;
    let _ = fileman.setup();

    let log: Log = Log::new(); // logs
    let cli: Args = Args::parse(); // cli

    // read myway_projects.json and convert to Vec<Project>
    let data = fileman.read()?;
    let mut json_data = serde_json::from_str::<ProjectList>(&data)?;

    let data_graveyard = fileman.read_graveyard()?;
    let mut graveyard_json_data = serde_json::from_str::<GraveyardList>(&data_graveyard)?;


    // initiate the match to analize command delivered
    match_cli(
        &cli.command, 
        log, 
        &mut fileman, 
        &mut json_data, 
        &mut graveyard_json_data
    )?;

    Ok(())

}
