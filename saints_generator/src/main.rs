use std::{thread, time::Duration};

use scraper::{Html, Selector};

fn get_saints(document: &Html) -> Vec<String> {
    let saint_selector = Selector::parse("div.mw-category-group a[title]").unwrap();

    document
        .select(&saint_selector)
        .into_iter()
        .map(|elem| elem.value().attr("title"))
        .flatten()
        .filter(|saint| !saint.starts_with("Categoria:"))
        .map(String::from)
        .collect()
}

fn find_next_url(document: &Html) -> Option<String> {
    let a_selector = Selector::parse("a[href]").unwrap();

    document
        .select(&a_selector)
        .into_iter()
        .filter_map(|elem| {
            if elem.inner_html() == "pagina successiva" {
                Some(elem.value().attr("href"))
            } else {
                None
            }
        })
        .flatten()
        .map(String::from)
        .next()
}

fn main() {
    let mut saints_page =
        String::from("https://it.wikipedia.org/w/index.php?title=Categoria:Santi_per_nome");

    loop {
        let output = reqwest::blocking::get(saints_page).unwrap().text().unwrap();

        let page = Html::parse_document(&output);

        let saints = get_saints(&page);

        for saint in saints {
            println!("{saint}");
        }

        let next_url = find_next_url(&page);

        if let Some(url) = next_url {
            let url = format!("https://it.wikipedia.org/{url}");
            eprintln!("crawling {url}");
            saints_page = url;
        } else {
            break;
        }

        thread::sleep(Duration::from_secs(1));
    }
}
