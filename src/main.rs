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
    let mut data: ProjectList = fileman.read(&fileman.mw_path.clone())?;
    let mut data_graveyard: GraveyardList = fileman.read(&fileman.graveyard_path.clone())?;


    // initiate the match to analize command delivered
    match_cli(
        &cli.command, 
        log, 
        &mut fileman, 
        &mut data, 
        &mut data_graveyard
    )?;

    Ok(())

}
