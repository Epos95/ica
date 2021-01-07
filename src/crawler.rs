
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate, Element};
use select::node::Node;

use ica::*;

pub fn normalize(s: String) -> String {
    let r = s.replace("\n", "");
    let v: Vec<String> = r.split(' ').map(|x| x.to_string()).collect();
    println!("{:?}", v);
    v.join(" ")


}

pub async fn get_items(dom: Document) -> Result<Vec<Item>, CrawlerErrors> {

    let res: Vec<Item> = vec![];

    for node in dom.find(Class("product-info-wrapper__body")).take(1) {

        let q: Vec<String> = node.find(Name("h4"))
            .collect::<Vec<Node>>()
            .iter_mut()
            .map(|x| x.text().replace("\n","").replace(" ", ""))
            .collect();

        let a: Vec<String> = node.find(Name("p"))
            .collect::<Vec<Node>>()
            .iter_mut()
            .map(|x| normalize(x.text()))
            .collect();

        for i in 0..q.len() {
            println!("{} {}", q.get(i).unwrap(), a.get(i).unwrap());
        }
        println!("\n");

        /*
        println!("starting h4");
        for h in node.find(Name("h4")) {
            println!("{}", h.text().replace("\n", ""));
        }
        println!("p starting now");

        for h in node.find(Name("p")) {
            println!("{}", h.text().replace("\n", ""));
        }
        println!();
        */
    }
    /*
    println!("prices:");
    for node in dom.find(Class("product-price__price-items-wrapper")) {
        //println!("{:?}", node.text().replace("\n", ""));
        let amount: Vec<Node> = node.find(Class("product-price__amount")).collect();
        if !amount.is_empty() {
            println!("{:?}", amount.get(0).unwrap().text());
        }

    }
    */




    Ok(vec!["".to_string()])
}

pub enum CrawlerErrors {

}
