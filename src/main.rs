// use std::collections::HashMap;
use std::io::Write;
use std::fs;

pub mod utils;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let url: &String = &args[1];
    let sitename: &String = &args[2];
    let files = utils::SiteFolder::new(sitename);

    println!("folder created {}", files.site_folder);

    let resp = get_site_content(url).await;
    match resp {
        Ok(data) => {
            let mut file = fs::File::create(files.index_file).expect("Unable to create file");
            file.write(data.as_bytes()).expect("Unable to write to file.");

            println!("Site data fetched...")
        },
        Err(err) => panic!("Problem fetching site data {:?}", err)
    };
}

async fn get_site_content(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url)
        .await?.text().await?;

    Ok(resp)
}