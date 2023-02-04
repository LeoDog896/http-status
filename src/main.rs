use jsonc_parser::parse_to_serde_value;
use clap::Parser;

const INFO_JSON: &str = include_str!("info.jsonc");

/// Get info about HTTP status codes
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct AppSettings {
    /// The HTTP status code to get info about
    code: u16,
}

#[derive(serde::Deserialize)]
struct HTTPInformation {
    code: u16,
    name: String,
    description: String,
    link: Option<String>,
}

fn main() {
    let settings = AppSettings::parse();

    let value = parse_to_serde_value(INFO_JSON, &Default::default()).unwrap().unwrap();

    let http_info: Vec<HTTPInformation> = serde_json::from_value(value).unwrap();

    let info = http_info.iter().find(|info| info.code == settings.code).unwrap();

    println!("{} {}\n\n{}", info.code, info.name, info.description);
}
