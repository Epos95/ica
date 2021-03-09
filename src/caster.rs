
use ica::*;
use clap::ArgMatches;
use crossterm::style::*;

pub fn show(
    items: Vec<Item>, matches: ArgMatches, config: Config
) -> Result<(), CasterErrors> {

    let mut res = String::new();


    // loop over all the strings in word_list
    for item in items {
        let mut found_match = false;
        for word in config.word_list.iter() {
             if (match_words(word, &item.name)) || 
                (match_words(word, &item.producer)) || 
                (match_words(word, &item.more_info)) {

                found_match = true;
                break;       
            }
        }

        if found_match || matches.is_present("all") {
            res += &format!("\n  {}, {}, {}, {}",
                          item.name,
                          item.producer,
                          item.mass,
                          item.deal,
                          );
        }
    }

    if res.is_empty() {
        return Err(CasterErrors::NoProductsFound);
    }

    println!("{}", res);

    Ok(())
}

pub enum CasterErrors {
    NoProductsFound
}

// maybe rewrite this to use proper search thing 
fn match_words(word: &String, to_search_in: &String) -> bool {
    to_search_in.to_lowercase().contains(word.to_lowercase().as_str())
}
