
use select::document::Document;
use select::predicate::{Class, Name};
use select::node::Node;

use ica::*;

pub async fn get_items(dom: Document, store_type: StoreTypes) -> Result<Vec<Item>, CrawlerErrors> {

    // the storetype given should decide how to parse the given document

    let mut res: Vec<Item> = vec![];

    let names: Vec<String> = dom.find(Class("offer-type__product-name"))
            .map(|x| normalize(x.text()))
            .collect();

    if names.is_empty() {
        return Err(CrawlerErrors::HTMLStructureError);
    }

    for (ctr, node) in dom.find(Class("product-info-wrapper__body")).enumerate() {

        let a: Vec<String> = node.find(Name("p"))
            .collect::<Vec<Node>>()
            .iter_mut()
            .map(|x| normalize(x.text()))
            .collect();

        res.push(
            Item::new(
                names.get(ctr).unwrap().to_string(),
                a.get(0).unwrap().to_string(),
                a.get(1).unwrap().to_string(),
                a.get(2).unwrap().to_string(),
                parse_more_info(a.get(2).unwrap().to_string())
            )
        );
    }

    Ok(res)
}

pub enum CrawlerErrors {
    HTMLStructureError
}

pub enum StoreTypes {
    ICA,
    COOP
}

fn normalize(s: String) -> String {
    let r = s.replace("\n", "");
    let v: Vec<String> = r.split(' ')
        .map(|x| x.to_string())
        .filter(|x| x != &"".to_string())
        .collect();

    v.join(" ")
}

fn parse_more_info(info: String) -> (bool, bool, String) {
    let mut res = (false, false, "".to_string());
    let mut v: Vec<String> = info.split(" ")
        .map(|x| x.to_string())
        .collect();

    if v.contains(&"Stammispris".to_string()) {
        res.0 = true;
        v.retain(|x| x != &"Stammispris".to_string());
    }

    if v.contains(&"Fryst".to_string()) && v.contains(&"Frysta.".to_string()) {
        res.1 = true;
        v.remove(0);
    }

    for (i, word) in v.iter().rev().enumerate() {
        if word == &"/st".to_string() || word == &"/+pant".to_string() || word == &"/kg".to_string() {
            res.2 = format!("{}/st", v.iter().rev().nth(i+1).unwrap());
            break;
        } else if word.contains(&":-".to_string()) {
            if v.iter().rev().nth(i+1).unwrap() == &"för".to_string() {
                res.2 = format!("{} för {}", v.iter().rev().nth(i+2).unwrap(), v.iter().rev().nth(i).unwrap());
                break;
            }
        } else if word.contains(&"/kg".to_string()) {
            res.2 = word.to_string();
        }
    }

    if res.2 == "".to_string() {
        res.2 = "Invalid info".to_string();
    }

    res
}
