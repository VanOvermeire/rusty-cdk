# IAM Docs Scraper

A Rust application that scrapes AWS IAM documentation to extract service actions and permissions from the official AWS Service Authorization Reference.

## Overview

This tool automatically extracts all available IAM actions for AWS services by scraping the [AWS Service Authorization Reference](https://docs.aws.amazon.com/service-authorization/latest/reference/reference_policies_actions-resources-contextkeys.html). 

It outputs the data in CSV format for easy integration with other tools.

## Prerequisites

- Rust 1.70+ (edition 2024)
- An `output` dir with an (emtpy) `iam.csv` file

## Usage

Run the scraper:

```bash
cargo run
```

The application will:
1. Fetch the main AWS Service Authorization Reference page
2. Extract links to individual service documentation pages
3. Scrape each service page for IAM actions
4. Output results to `output/iam.csv`

## Output Format

The output CSV file contains service names followed by their associated IAM actions, separated by commas:

```
service_name,action1,action2,action3,...
```

Example:
```
s3,AbortMultipartUpload,CreateBucket,DeleteObject,GetObject,...
```
