#![warn(unused_extern_crates)]
use clap::{Arg, App};
use std::io;
use std::io::BufRead;
use crossterm::style::*;

mod reader;
mod networker;
mod crawler;
mod caster;
mod creator;

#[tokio::main]
async fn main() {
    let matches = App::new("ica shopping program")
        .version("1.0")
        .author("Max Agnesund <maxagnesund95ATgmailDOTcom")
        .about("Helps you find discounted grocceries")
        .arg(Arg::new("config")
             .about("Path to a custom config file")
             .short('c')
             .long("config")
             .value_name("FILE")
             .takes_value(true))
        .arg(Arg::new("all")
             .about("Print out everything that is on sale, not just stuff that \
                    matches the given config file")
             .short('a'))
        .arg(Arg::new("dump")
             .about("Dumps output as json")
             .long("dump")
             .short('d'))
        .arg(Arg::new("url")
             .about("The url of the store to check out")
             .short('u')
             .long("url")
             .takes_value(true)
             .value_name("STORE"))
        .arg(Arg::new("create")
            .about("Helps the user create a config file in a specific place")
            .long("create")
            .takes_value(false))
        .get_matches();


    if matches.is_present("create") {
        match creator::create_config() {
            Ok(s) => {

            },
            Err(creator::CreatorErrors::HomeNotFound) => {
                println!("  {}{}",
                        "Error".red(),
                        ": Home directory not found.");
            },
            Err(creator::CreatorErrors::FileAlreadyExists) => {
                println!("  {}{}",
                        "Error".red(),
                        ": File already exists.");
            }
            _ => {
                unimplemented!();
            }
        }
        std::process::exit(0);
    }

    // errors regarding listing everything needs to be fixed
    // specifically that we cant list everything without a valid config

    // solution:
    // if we fail to read the config when the "all" option is set
    // we should create a dummy object while asking the user for a url 

    // get config struct
    let config = match reader::get_config(matches.clone()) {
        Ok(s) => s,
        Err(reader::ReaderErrors::CouldntReadFile) => {
            println!("   {}{}",
                     "Error".red(),
                     ": Could not read the file");
                    

            // if the user specified all then not having a config is fine
            if matches.is_present("all") {
                get_dummy_config()
            } else {
                std::process::exit(0);
            }

        },
        Err(reader::ReaderErrors::InvalidConfigFile) => {
            println!("   {}{}",
                     "Error".red(),
                     ": Invalid config file format.");
                    

            // if the user specified all then not having a config is fine
            if matches.is_present("all") {
                get_dummy_config()
            } else {
                std::process::exit(0);
            }
        }
    };

    let selected_store = if matches.is_present("url") {
        matches.value_of("url").unwrap().to_string()
    } else {
        match config.urls.len() {
            0 => {
                println!("   {}{}",
                        "Error".red(),
                        ": No stores listed in config file.");
                std::process::exit(0);
            },
            1 => config.urls[0].clone(),
            _ => {
                // more than one store listed

                println!(" Choose store url to use:");
                for (i, store) in config.urls.iter().enumerate() {
                    println!("  {}: {}",
                             (i+1).to_string().bold(),
                             store);
                }

                let mut line = String::new();
                let stdin = io::stdin();
                stdin.lock().read_line(&mut line).expect("Could not read line");

                
                let mut inte = match line.trim().parse::<i32>() {
                    Ok(s)  => {
                        if s > 0 {
                            s-1
                        } else{
                            println!("invalid input, defaulting to first url");
                            0
                        }
                    },
                    Err(_) => {
                        //println!("{}", e);
                        println!("invalid input, defaulting to first url");
                        0
                    }
                };

                if config.urls.len() <= inte as usize {
                    println!("invalid input, defaulting to first url");
                    inte = 0;
                }

                config.urls[inte as usize].clone()
            }
        }
    };

    println!("   {}{}\n     {}",
            "Ok".green(),
            ": Using store url: ",
            selected_store.to_string());

    let document = match networker::get_dom(&selected_store).await {
        Ok(s) => {
            println!("   {}{}",
                    "Ok".green(),
                    ": Downloaded HTML document.");
            s
        }
        Err(networker::NetworkerErrors::NetworkError) => {
            println!("   {}{}",
                    "Error".red(),
                    ": Could not connect to website");
            std::process::exit(0);
        },
        Err(networker::NetworkerErrors::ConversionError) => {
            println!("   {}{}",
                    "Error".red(),
                    ": Could not get text of HTML");
            std::process::exit(0);
        },
        //_ => { panic!("unimplemented error"); store: String}
    };

    let items = match crawler::get_items(document, get_store_type(selected_store)).await {
        Ok(s) => s,
        Err(crawler::CrawlerErrors::HTMLStructureError) => {
            println!("   {}{}",
                    "Error".red(),
                    ": HTML did not match exected website.");
            std::process::exit(0);
        }
        //_ => { panic!("unimplemented error"); }
    };


    println!("   {}{}",
            "Ok".green(),
            ": Retrived items from document");

    if let Err(caster::CasterErrors::NoProductsFound) = caster::show(items, matches, config) {
        println!("   {}{}",
                "Info".yellow(),
                ": No matchning productes were found!");
    }
}


// This function will be useful when extending functionality to other stores 
// besides ica, it is meant to get the type of website to make scraping easier
fn get_store_type(store: String) -> crawler::StoreTypes {
    return crawler::StoreTypes::ICA;
}

// this function is meant to return a dummy config to be used when the "all" option 
// is passed
fn get_dummy_config() -> ica::Config {
    let urls: Vec<String> = vec![];

    // ask the user for url via input here
    ica::Config::new(Vec::new(), urls)
}