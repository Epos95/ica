
use ica::*;
use clap::ArgMatches;
use termion::*;

pub fn show(
    items: Vec<Item>, matches: ArgMatches, config: Config
) -> Result<(), CasterErrors> {

    let mut res = String::new();


    // redesign this so that it uses a proper regex
    // loop over all the strings in word_list
    for item in items {
        // use fuzzy match for the string here
        // skriv egen algoritm för det

        // match on everything
        let mut found_match = false;
        for word in config.word_list.iter() {
            if match_words(word.to_string(), item.name.clone()) {
                found_match = true;
                break; 
            }
        }

        if found_match || matches.is_present("all") {
            res += &format!("\n{}, {}, {}, {}",
                          item.name,
                          item.producer,
                          item.mass,
                          item.deal
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

fn match_words(word: String, to_search_in: String) -> bool {
    // if {word} exists in some capacity in {to_search_in}, true
    true
}
