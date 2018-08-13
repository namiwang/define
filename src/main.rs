use std::collections::HashMap;

extern crate clap;
use clap::{App, Arg};

extern crate reqwest;

// extern crate select;
// use select::document::Document;
// use select::predicate::{Predicate, Attr, Class, Name};

extern crate scraper;
use scraper::{Html, Selector};


fn main() {
    let matches = App::new("Define")
                        .version("0.1.0")
                        .author("Nami W. <namiheike@gmail.com>")
                        .about("Utility to lookup word")
                        .arg(Arg::with_name("word")
                            .help("The word to define")
                            .index(1))
                        .get_matches();


    let word = matches.value_of("word").expect("no word provided");

    println!("Looking up for word: {}", word);

    let dict = "google";
    match dict {
        "google" => query_from_google(word),
        _ => panic!("unreachable")
    }

}

fn query_from_google(word: &str) {
    let query_url = &format!("https://www.google.com/search?q=define+{}", word);

    let client = reqwest::Client::new();
    let response: &str = &client.get(query_url)
        .header(reqwest::header::UserAgent::new("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/60.0.3112.113 Safari/537.36"))
        .send().expect("connection failed")
        .text().expect("connection failed");

    let _document = Html::parse_document(response);
    let body = _document.select( &Selector::parse("body").unwrap() ).next().unwrap();

    // let mut define: HashMap<&str, &str> = HashMap::new();

    // define.insert( "word",
    //     body.select( &Selector::parse("div.dDoNo span").unwrap() ).next().unwrap().text().next().unwrap()
    // );

    // define.insert( "proununciation",
    //     body.select( &Selector::parse("div.dDoNo span").unwrap() ).next().unwrap().text().next().unwrap()
    // );

    // let phonetic = Vec::new();
    let phonetic: Vec<String> = body.select( &Selector::parse(".lr_dct_ph.XpoqFe").unwrap() ).next().unwrap().children()
        .filter(|phonetic_span|{
            
        })
        .map(|phonetic_span|{
    });

    }
    //          $(".lr_dct_ph.XpoqFe").first().find('span').each(function(i, element){
    //             dictionary.phonetic.push($(this).text()); 
    //          });
    // body.

    // println!("{:?}", phonetic);


    // let selector = Selector::parse("li").unwrap();

    // let document = Document::from(response);
    // let body = document.find(Name("body"));

    //          dictionary.word = $("div.dDoNo span").first().text();
    //          dictionary.pronunciation = "https:" + $('.lr_dct_spkr.lr_dct_spkr_off audio')[0].attribs.src;
    //          dictionary.pronunciation = dictionary.pronunciation.replace('--_gb', '--_us');
    //          dictionary.phonetic = [];
    //          $(".lr_dct_ph.XpoqFe").first().find('span').each(function(i, element){
    //             dictionary.phonetic.push($(this).text()); 
    //          });
    //          dictionary.meaning = {};

}

pub fn main2(document: &str) {
    // stackoverflow.html was fetched from
    //  on
    // Aug 10, 2015.
    // let document = Document::from(include_str!("stackoverflow.html"));


    // println!("# Menu");
    // for node in document.find(Attr("id", "hmenus").descendant(Name("a"))) {
    //     println!("{} ({:?})", node.text(), node.attr("href").unwrap());
    // }
    // println!("");

    // println!("# Top 5 Questions");
    // for node in document.find(Class("question-summary")).take(5) {
    //     let question = node.find(Class("question-hyperlink")).next().unwrap();
    //     let votes = node.find(Class("vote-count-post")).next().unwrap().text();
    //     let answers = node.find(Class("status").descendant(Name("strong")))
    //         .next()
    //         .unwrap()
    //         .text();
    //     let tags = node.find(Class("post-tag")).map(|tag| tag.text()).collect::<Vec<_>>();
    //     let asked_on = node.find(Class("relativetime")).next().unwrap().text();
    //     let asker = node.find(Class("user-details").descendant(Name("a")))
    //         .next()
    //         .unwrap()
    //         .text();
    //     println!(" Question: {}", question.text());
    //     println!("  Answers: {}", answers);
    //     println!("    Votes: {}", votes);
    //     println!("   Tagged: {}", tags.join(", "));
    //     println!(" Asked on: {}", asked_on);
    //     println!("    Asker: {}", asker);
    //     println!("Permalink: http://stackoverflow.com{}",
    //              question.attr("href").unwrap());
    //     println!("");
    // }

    // println!("# Top 10 Related Tags");
    // for node in document.find(Attr("id", "h-related-tags"))
    //     .next()
    //     .unwrap()
    //     .parent()
    //     .unwrap()
    //     .find(Name("div"))
    //     .take(10) {
    //     let tag = node.find(Name("a")).next().unwrap().text();
    //     let count = node.find(Class("item-multiplier-count")).next().unwrap().text();
    //     println!("{} ({})", tag, count);
    // }
}
