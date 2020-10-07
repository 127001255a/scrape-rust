#[macro_use] extern crate prettytable;

use error_chain::error_chain;
use select::document::Document;
use select::predicate::{Name, Class, Predicate};
use scraper::{Html, Selector};
use prettytable::Table;


error_chain! {
      foreign_links {
          ReqError(reqwest::Error);
          IoError(std::io::Error);
      }
}

#[tokio::main]
async fn main() -> Result<()> {
    let res = reqwest::get("https://www.rust-lang.org/en-US/")
        .await?
        .text()
        .await?;

    Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{}", x));

    let mut body = reqwest::get("https://news.ycombinator.com").await?.text().await?;
    let fragment = Html::parse_document(&body);
    // parses based on a CSS selector
    let stories = Selector::parse(".storylink").unwrap();

    // iterate over elements matching our selector
    for story in fragment.select(&stories) {
        // grab the headline text and place into a vector
        let story_txt = story.text().collect::<Vec<_>>();
        println!("{:?}", story_txt);
    }

    // New
    let resp1 = reqwest::get("https://news.ycombinator.com").await?.text().await?;

    let document = Document::from(resp1.as_str());

    // finding all instances of our class of interest
    for node in document.find(Class("athing")) {
        // grabbing the story rank
        let rank = node.find(Class("rank")).next().unwrap();
        // finding class, then selecting article title
        let story = node.find(Class("title").descendant(Name("a")))
            .next()
            .unwrap()
            .text();
        // printing out | rank | story headline
        println!("\n | {} | {}\n", rank.text(), story);
        // same as above
        let url = node.find(Class("title").descendant(Name("a"))).next().unwrap();
        // however, we don't grab text
        // instead find the "href" attribute, which gives us the url
        println!("{:?}\n", url.attr("href").unwrap());
    }

    //Last ?
    let resp2 = reqwest::get("https://news.ycombinator.com").await?.text().await?;

    let document = Document::from(resp2.as_str());

    let mut table = Table::new();
    // same as before
    for node in document.find(Class("athing")) {
        let rank = node.find(Class("rank")).next().unwrap();
        let story = node.find(Class("title").descendant(Name("a")))
            .next()
            .unwrap()
            .text();
        let url = node.find(Class("title").descendant(Name("a")))
            .next()
            .unwrap();
        let url_txt = url.attr("href").unwrap();
        // shorten strings to make table aesthetically appealing
        // otherwise table will look mangled by long URLs
        let url_trim = url_txt.trim_left_matches('/');
        let rank_story = format!(" | {} | {}", rank.text(), story);
        // [FdBybl->] specifies row formatting
        // F (foreground) d (black text)
        // B (background) y (yellow text) l (left-align)
        table.add_row(row![FdBybl->rank_story]);
        table.add_row(row![Fy->url_trim]);
    }
    // print table to stdout
    table.printstd();

    Ok(())
}