use anyhow::{Context, Ok, Result};
use reqwest::blocking::Client;
use reqwest::header::USER_AGENT;
use scraper::selectable::Selectable;
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::fs;
use std::fs::read_to_string;

const BASE_URL: &str = "https://docs.aws.amazon.com/AWSCloudFormation/latest/TemplateReference";

/// Retrieves raw, but parsable (CSV), resource info from the AWS docs
/// Output is written to the output dir
/// Works for Resources but not for 'helpers' (custom props) -> skip the outer loop and add those urls to `retrieve_resource_props`
fn main() -> Result<()> {
    let client = Client::new();

    let mut output = vec![];
    
    let input = read_to_string("./output/list_of_urls")?;
    let resource_urls = input.split("\n").filter(|v| !v.is_empty());

    for url in resource_urls {
        let resources = get_specific_resource(&client, url)?;
    
        for r in resources {
            let (name, props) = retrieve_resource_props(&client, &r)?;
            let joined_props = props
                .into_iter()
                .map(|v| format!("{}==={}", v.0, v.1.join("###")))
                .collect::<Vec<_>>()
                .join(";");
            let output_for_single_resource = format!("{};{}", name, joined_props);
            output.push(output_for_single_resource);
        }
    }

    fs::write("output/raw_resources.csv", output.join("\n").as_bytes())?;

    Ok(())
}

fn retrieve_resource_props(client: &Client, path: &str) -> Result<(String, HashMap<String, Vec<String>>)> {
    let resource_url = format!("{}/{}", BASE_URL, path);
    println!("Retrieving props for {}", resource_url);
    let resource_html = client
        .get(&resource_url)
        .header(
            USER_AGENT,
            "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36",
        )
        .send()
        .context(format!("getting resource props url failed: {}", resource_url))?
        .text()?;

    let document = Html::parse_document(&resource_html);

    let name_selector = Selector::parse("h1").unwrap();
    let name = document.select(&name_selector).next();
    let name = name.context("resource should have a name")?.inner_html();

    let variable_lists_selector = Selector::parse(".variablelist > dl").unwrap();
    // first list is the props
    let variable_list = document.select(&variable_lists_selector).next().context("resource should have a list of props")?;
    let prop_name_selector = Selector::parse("dt > span > code").unwrap();
    let mut prop_names = variable_list.select(&prop_name_selector);

    let mut names = vec![];

    while let Some(name) = prop_names.next() {
        let name = name.inner_html();
        names.push(name);
    }

    let prop_info_selector = Selector::parse("dd > p").unwrap();
    let mut prop_info = variable_list.select(&prop_info_selector);

    let mut names_iter = names.into_iter();
    let mut current_name = names_iter.next();
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut gather_info = vec![];

    while let Some(info) = prop_info.next() {
        let name = info.inner_html();
        let name_without_em = name.replace("<em>", "").replace("</em>", "").replace("\n", "");

        if name_without_em.starts_with("Update") {
            // this also means we don't add the Update info (currently not needed)
            map.insert(current_name.take().context("should be a prop name for every collection of info")?, gather_info.drain(..).collect());
            current_name = names_iter.next();
        } else {
            gather_info.push(name_without_em);
        }
    }

    Ok((name, map))
}

fn get_specific_resource(client: &Client, suffix: &str) -> Result<Vec<String>> {
    let resource_url = format!("{}/{}", BASE_URL, suffix);
    println!("Retrieving info for {}", resource_url);
    let resource_html = client
        .get(&resource_url)
        .header(
            USER_AGENT,
            "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/51.0.2704.103 Safari/537.36",
        )
        .send()
        .context(format!("getting resource url failed: {}", resource_url))?
        .text()?;

    let document = Html::parse_document(&resource_html);
    let main_selector = Selector::parse("#main-col-body > div.itemizedlist > ul > li > p > a").unwrap();
    let mut main = document.select(&main_selector);
    let mut resources = vec![];

    while let Some(el) = main.next() {
        let href = el.attr("href").context("a element should have href")?.to_string();
        let href = href.replace("./", "");
        resources.push(href);
    }

    Ok(resources)
}
