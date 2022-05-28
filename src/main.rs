// use std::collections::HashMap;
use std::io::Write;
use std::fs;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let url: &str = &args[1];
    let resp = get_site_content(url).await;
    match resp {
        Ok(data) => {
            let mut file = fs::File::create("sites/test.html").expect("Unable to create file");
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