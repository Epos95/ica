use clap::{Arg, App};
use std::io;
use std::io::BufRead;
use crossterm::style::*;

mod reader;
mod networker;
mod crawler;
mod caster;

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
        .arg(Arg::new("url")
             .about("The url of the store to check out")
             .short('u')
             .long("url")
             .takes_value(true)
             .value_name("STORE"))
        .get_matches();


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

    let store_url = if matches.is_present("url") {
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
            store_url.to_string());

    let document = match networker::get_dom(&store_url).await {
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

    let items = match crawler::get_items(document).await {
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

// this function is meant to return a dummy config to be used when the "all" option 
// is passed
fn get_dummy_config() -> ica::Config {
    let urls: Vec<String> = vec![];

    // ask the user for url via input here
    ica::Config::new(Vec::new(), urls)
}