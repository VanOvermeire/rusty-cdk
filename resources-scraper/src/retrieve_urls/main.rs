use anyhow::{Context, Ok, Result};
use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use scraper::selectable::Selectable;
use scraper::{Html, Selector};
use std::fs;

const BASE_URL: &str = "https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference";
const START_SUFFIX: &str = "aws-template-resource-type-ref.html";

// add resource groupings that have been completely added
const IGNORE_LIST: [&str; 3] = ["cfn-reference-shared.html", "AWS_SNS.html", "AWS_SQS.html"];

/// Retrieve a list of relative URLs that point to resource grouping like S3
/// Will retrieve some
fn main() -> Result<()> {
    let client = Client::new();
    let output = find_resource_categories(&client)?;
    fs::write("output/list_of_urls", output.join("\n").as_bytes())?;

    Ok(())
}

fn find_resource_categories(client: &Client) -> Result<Vec<String>> {
    let start_url = format!("{}/{}", BASE_URL, START_SUFFIX);
    let resource_html = client
        .get(&start_url)
        .header(
            USER_AGENT,
            "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36",
        )
        .send()
        .context(format!("getting resource url failed: {}", start_url))?
        .text()?;

    let document = Html::parse_document(&resource_html);
    let list_selector = Selector::parse("ul").unwrap();
    let list_element_selector = Selector::parse("li > p > a").unwrap();
    let mut list = document.select(&list_selector);
    let first_list = list.next();

    let mut list_elements = first_list.unwrap().select(&list_element_selector);
    let mut hrefs = vec![];

    while let Some(el) = list_elements.next() {
        let href = el.attr("href").unwrap();
        let href = href.replace("./", "");

        if !IGNORE_LIST.contains(&href.as_str()) {
            hrefs.push(href);
        }
    }

    Ok(hrefs)
}
