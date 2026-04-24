use std::env;
use std::io::{Read, Write};
use std::path::Path;

/* helper func to download input text */
pub fn fetch_cache_input_text(year: u16, day: u16) -> std::io::Result<String> {
    {
        let dirname = format!("./inputs/{}", year);
        let directory = Path::new(&dirname);
        std::fs::create_dir_all(directory)?;
    }

    let filename = format!("./inputs/{}/{}.txt", year, day);
    let filepath = Path::new(&filename);
    if filepath.exists() {
        let mut file = match std::fs::File::open(filepath) {
            Ok(file) => file,
            Err(err) => return Err(std::io::Error::other(err)),
        };
        let mut content = String::new();
        return match file.read_to_string(&mut content) {
            Ok(_) => Ok(content),
            Err(err) => Err(std::io::Error::other(err)),
        };
    }

    // we do not have a cached file => fetch the data via http
    let content = http_fetch_input_text(year, day);
    let mut file = std::fs::File::create(filepath)?;
    file.write_all(content.as_bytes())?;
    Ok(content)
}

fn http_fetch_input_text(year: u16, day: u16) -> String {
    let session_id = match env::var("ADVENT_AUTH_SESSION_ID") {
        Ok(s) => s,
        Err(err) => panic!("ADVENT_AUTH_SESSION_ID not set, {}", err),
    };
    let cookie = format!("session={}", session_id);
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let client = reqwest::blocking::Client::new();
    let res = client.get(url).header("Cookie", cookie).send().unwrap();
    res.text().unwrap()
}
