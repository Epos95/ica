// this should be the entry point
//use select::document::Document;
//use select::predicate::Class;
use clap::{Arg, App};
use std::io;
use std::io::BufRead;
use termion::*;

mod reader;
mod networker;
mod crawler;

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
             .about("Print out everything that is on sale, not just stuff that matches the given config file")
             .short('a'))
        .arg(Arg::new("dump")
             .about("Dumps output as json")
             .long("dump")
             .short('d'))
        .arg(Arg::new("store")
             .about("Part of the url of the store to be checked")
             .short('s')
             .long("store")
             .takes_value(true)
             .value_name("STORE"))
        .get_matches();

    // get config struct
    let config = match reader::get_config(matches) {
        Ok(s) => s,
        Err(reader::ReaderErrors::CouldntReadFile) => {
            println!("   {}Error{}: couldnt read the file",
                     color::Fg(color::Red),
                     color::Fg(color::Reset));
            std::process::exit(0);
        },
        Err(reader::ReaderErrors::InvalidConfigFile) => {
            println!("   {}Error{}: invalid config file format",
                     color::Fg(color::Red),
                     color::Fg(color::Reset));
            std::process::exit(0);
        }
    };

    let selected_store = match config.urls.len() {
        0 => {
            println!("   {}Error{}: no stores listed in config file.",
                        color::Fg(color::Red),
                        color::Fg(color::Reset));
            std::process::exit(0);
        },
        1 => config.urls[0].clone(),
        _ => {
            // more than one store listed

            println!(" Choose store url to use:");
            for (i, store) in config.urls.iter().enumerate() {
                println!("  {}{}{}: {}",
                         style::Bold,
                         i+1,
                         style::Reset,
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
                        println!("invalid sdfginput, defaulting to first url");
                        0
                    }
                },
                Err(e) => {
                    println!("{}", e);
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
    };

    let document = match networker::get_dom(selected_store).await {
        Ok(s) => s,
        Err(networker::NetworkerErrors::NetworkError) => {
            println!("   {}Error{}: Could not connect to website.",
                        color::Fg(color::Red),
                        color::Fg(color::Reset));
            std::process::exit(0);
        },
        Err(networker::NetworkerErrors::ConversionError) => {
            println!("   {}Error{}: Could not get text of HTML.",
                        color::Fg(color::Red),
                        color::Fg(color::Reset));
            std::process::exit(0);

        }
        _ => { panic!("unimplemented error"); }
    };

    let items = match crawler::get_items(document).await {
        Ok(s) => s,
        _ => { panic!("unimplemented error"); }
    };

    println!("{:?}", items);

    // get the document of the web page

    // analyze the document and get the data

    // compare the data to the users config and what things they pay attention too

    // print out nicely or dump as json
}
