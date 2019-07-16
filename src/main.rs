#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
extern crate toml;
#[macro_use]
extern crate clap;

use std::process::exit;
use std::path::Path;
use crate::script::{ForgeScript, ScriptTask};
use simplelog::{CombinedLogger, TermLogger, WriteLogger, Config};
use log::LevelFilter;
use std::fs::File;
use clap::{App, AppSettings, Arg, SubCommand};

mod script;


fn main() {
    // Setup logging
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default()).unwrap(),
            WriteLogger::new(LevelFilter::Debug, Config::default(), File::create("forge.log").unwrap()),
        ]
    ).unwrap();

    // Parse CLI
    let matches = App::new("forge")
        .name("Forge")
        .bin_name("forge")
        .about("Forge script runner")
        .version(crate_version!())
        .usage("forge <task>")
        .setting(AppSettings::DisableHelpSubcommand)
        .setting(AppSettings::ArgRequiredElseHelp)
        .arg(Arg::with_name("script")
            .long("script")
            .short("s")
            .value_name("file")
            .help("The yaml script to execute, default to forge.yml"))
        .arg(Arg::with_name("list")
            .long("list")
            .help("Lists all available tasks"))
        .arg(Arg::with_name("task")
            .help("The task name to execute")
            .required(true)
            .conflicts_with("list"))
        .get_matches();

    // Print about line
    info!("Forge - {}", crate_version!());

    // Parse script arg
    let script_path = Path::new(matches.value_of("script").unwrap_or("forge.yml"));
    info!("Script: {}", script_path.display());
    if !script_path.exists() {
        error!("Script does not exist");
        exit(1);
    }

    // Read config
    let script = ForgeScript::from_file(&script_path);
}
