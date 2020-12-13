
// This grabs the JSON content from decksofkeyforge.com for a list of keyforge decks
// TODO: Add a 25 and wait-1-minute feature as they request limit at that.
// TODO: Move the filenames to be command line args

extern crate reqwest;

use std::fs;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use reqwest::Error;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};


fn main() -> Result<(), Error> {
    // load api key
    let tmp_api_key = fs::read_to_string("../api.key")
        .expect("Something went wrong reading the apikey file");
    let api_key = tmp_api_key.trim();
    println!("api.key loaded");

    // load deck list
    let decks = lines_from_file("../decks.txt");
    println!("deck list loaded");

    // save directory
    let save_dir = "../decks/";

    // get deck list
    save_decks(api_key, decks, save_dir);

    Ok(())
}

// https://stackoverflow.com/questions/30801031/read-a-file-and-get-an-array-of-strings#comment49651189_30801031
fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn save_decks(api_key: &str, decks: Vec<String>, save_dir: &str) -> Result<(), Error> {
    let mut count = 1;
    // loop over decks
    for deck in decks {
        if count <= 25 {
            let saved = save_deck(api_key, &deck, save_dir)?;
            if saved {
                count += 1;
            }
        }
    }

    Ok(())
}

fn save_deck(api_key: &str, deck: &str, save_dir: &str) -> Result<bool, Error> {
    let save_path = Path::new(save_dir).join(deck);
    if save_path.exists() {
        println!("Skipping: {}", save_path.to_string_lossy());
        return Ok(false);
    }

    let request_url = format!("https://decksofkeyforge.com/public-api/v3/decks/{}", deck);
    println!("url: {}", request_url);

    fn construct_headers(api_key: &str) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));
        headers.insert("Api-Key", HeaderValue::from_str(api_key).unwrap());
        headers
    }

    let client = Client::new();
    let get_result = client.get(&request_url).headers(construct_headers(api_key)).send();

    let response = match get_result {
        Ok(response) => response,
        Err(error) => {
            println!("Error calling url: {}", error);
            return Err(error);
        },
    };

    if response.status().is_success() {
        fs::write(&save_path, response.text()?).expect("Unable to write file");
        println!("Saved: {}", save_path.to_string_lossy());
    } else if response.status().is_server_error() {
        println!("server error! Status: {:?}", response.status());
    } else {
        println!("Something else happened. Status: {:?}", response.status());
        println!("Text: {}", response.text()?);
    }


    Ok(true)
}
