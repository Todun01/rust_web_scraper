use scraper;
use reqwest::blocking::get;

fn main() {
    let test_link = get("https://tau.edu.ng")?.text()?;

    let document = scraper::Html::parse_document(test_link);
    let h3_selector = scraper::Selector::parse("h4").unwrap();

    if let Some(h3_element) = document.select(&h3_selector).next(){
        let h3_text = h3_element.text().collect::<String>();
        println!("{:?}", h3_text);
    } else{
        println!("No tag found");
    }
}
