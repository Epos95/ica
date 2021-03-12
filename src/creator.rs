// this module should help the user create a config file
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::fs::*;
use ica::*;
use crossterm::style::*;

pub fn create_config() -> Result<String, CreatorErrors> {
    println!(" Setting up config file.");
    println!("  Filepath? (empty for default config filepath");

    let mut filename = String::new(); 
    let stdin = io::stdin();
    stdin.lock().read_line(&mut filename).expect("no io.");


    if filename.is_empty() {
        match dirs::home_dir() {
            Some(s) => {
                filename = format!("{}/.config/ica/config.json",s.into_os_string().into_string().expect("Failed to convert to string"));
            },
            None => {
                return Err(CreatorErrors::HomeNotFound);
            }
        }
    }

    // check if file already exists
    if Path::new(&filename).exists() {
        // config (or just) file already exists
        return Err(CreatorErrors::FileAlreadyExists);
    }

    // something is wrong with the way we create files
    // create file
    let mut fp = if let Ok(s) = create_dir_all(&filename) {
        s
    } else {
        return Err(CreatorErrors::CouldntCreateFile)
    };

    let mut words: Vec<String> = vec![];
    let mut urls:  Vec<String> = vec![];

    let stdin = io::stdin();

    // get keywords
    println!("  Add keywords to config: (empty to exit)");
    let mut buf = String::from(" ");
    while buf != "" {
        stdin.lock().read_line(&mut buf).expect("no io.");
        println!("Added {} to config.",
            buf.clone().italic());
        words.push(buf.clone());
    }

    // get store urls
    println!("  Add store urls to config: (empty to exit)");
    buf = " ".to_string();
    while buf != "" {
        stdin.lock().read_line(&mut buf).expect("no io.");
        println!("Added {} to config.",
            buf.clone().italic());
        words.push(buf.clone());
    }



    Ok("".to_string())
}

pub enum CreatorErrors {
    HomeNotFound,
    FileAlreadyExists,
    CouldntCreateFile
}