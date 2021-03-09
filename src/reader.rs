
use ica::*;
use clap::ArgMatches;
use std::fs;
use crossterm::style::*;

// entry point
pub fn get_config(matches: ArgMatches) -> Result<Config, ReaderErrors> {

    // get the filename/path
    let filename: String = if matches.is_present("config") {
        matches.value_of("config").unwrap().to_string()
    } else {
        let home = dirs::home_dir()
            .expect("Apparently home_dir() failed? Guess your computer just does not \
                     have a home.")
            .into_os_string()
            .into_string()
            .expect("Somehow it failed to convert to string, if you meant for this \
                     to happen please tell me how");
        format!("{}/.config/ica/config.json", home)
    };

    // read from file
    let s: String = match fs::read_to_string(&filename) {
        Ok(s) => s,
        _ => { return Err(ReaderErrors::CouldntReadFile); }
    };

    // convert to json
    let json: Config = match serde_json::from_str(&s) {
        Ok(s) => s,
        _ => { return Err(ReaderErrors::InvalidConfigFile); }
    };

    println!("   {}{}{}",
             "Ok".green(),
             ": Found config file at ",
             filename);

    Ok(json)
}

pub enum ReaderErrors {
    CouldntReadFile,
    InvalidConfigFile,
}
