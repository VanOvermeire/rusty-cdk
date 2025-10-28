use anyhow::{Context, Result};
use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use scraper::selectable::Selectable;
use scraper::{Html, Selector};
use std::fs;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

const START_URL: &str = "https://docs.aws.amazon.com/service-authorization/latest/reference/reference_policies_actions-resources-contextkeys.html";
const BASE_SERVICE_URL: &str = "https://docs.aws.amazon.com/service-authorization/latest/reference";

fn main() -> Result<()> {
    let client = Client::new();

    let all_service_links = retrieve_all_services(&client)?;

    let output = all_service_links.par_iter().map(|l| {
        println!("Retrieving info for {}", l);
        retrieve_service(&client, l)
    }).collect::<Result<Vec<_>>>()?;

    fs::write("output/iam.csv", output.join("\n").as_bytes())?;

    Ok(())
}

fn retrieve_service(client: &Client, link: &String) -> Result<String> {
    let service_html = client.get(format!("{}/{}", BASE_SERVICE_URL, link))
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36")
        .send()
        .context(format!("getting service url failed: {}", link))?
        .text()?;

    let document = Html::parse_document(&service_html);
    let code_selector = Selector::parse("code").unwrap();
    let mut service_code = document.select(&code_selector);
    let service_code = service_code.next().context(format!("expected at least one <code> tag with the name of the service for {}", link))?.inner_html();

    let table_selector = Selector::parse("table > tbody > tr").unwrap();
    let td_selector = Selector::parse("td > a").unwrap();
    let mut table = document.select(&table_selector);
    let mut actions = vec![];

    while let Some(el) = table.next() {
        let mut tds = el.select(&td_selector);

        if let Some(first_td) = tds.next() {
            actions.push(first_td.inner_html());
        }
    }

    let actions = actions.join(",");
    let code_plus_actions = format!("{};{}", service_code, actions);

    Ok(code_plus_actions)
}

fn retrieve_all_services(client: &Client) -> Result<Vec<String>> {
    let start_html = client.get(START_URL)
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36")
        .send()
        .context(format!("getting start url failed: {}", START_URL))?
        .text()?;
    
    let document = Html::parse_document(&start_html);
    let link_selector = Selector::parse("a").unwrap();
    
    let mut selected = document.select(&link_selector);
    
    let mut all_services = vec![];
    
    while let Some(link) = selected.next() {
        if let Some(href) = link.attr("href") {
            if href.starts_with("./list") {
                all_services.push(href.to_string());
            }
        }
    }

    Ok(all_services)
}