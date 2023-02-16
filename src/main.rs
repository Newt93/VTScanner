use std::fs::File;
use std::io::Read;
use reqwest::multipart::reader;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("9a1eaad027f6c0fdfba5af602aadcb364caae4c568fbdbb2162e2e445caa7d3a").unwrap();
    let file_path = "/home/tyler/Documents/RustProjects/virustotal_filescanner/test.png";
    let mut buffer = Vec::new();
    let mut file = File::open(file_path)?;
    file.read_to_end(&mut buffer)?;
    let cursor = std::io::Cursor::new(buffer);

    let client = reqwest::Client::new();
    let response = client
        .post(&format!(
            "https://www.virustotal.com/vtapi/v2/file/scan?apikey={}",
            api_key
        ))
        .multipart(reqwest::multipart::Form::new().part("file", reqwest::multipart::reader::CursorReader::new(cursor)))
        .send()
        .await?;
    let json = response.text().await?;
    println!("{}", json);

    let resource = json
        .as_str()
        .unwrap()
        .replace("{", "")
        .replace("}", "")
        .replace("\"", "");

    let response = client
        .get(&format!(
            "https://www.virustotal.com/vtapi/v2/file/report?apikey={}&resource={}",
            api_key, resource
        ))
        .send()
        .await?;
    let json = response.text().await?;
    println!("{}", json);

    Ok(())
}
