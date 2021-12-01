pub type Error = http_req::error::Error;

fn generate_path(day: u32) -> std::path::PathBuf {
    ["input", &day.to_string()].iter().collect()
}
fn download_input(day: u32) -> Result<(), Error> {
    let session = std::fs::read_to_string("cookie")?;
    let cookie = format!("session={}", session);
    let url = format!("https://adventofcode.com/{}/day/{}/input", 2021, day);
    let url_str: &str = &url;
    let uri = http_req::uri::Uri::try_from(url_str).unwrap();

    let path = generate_path(day);
    let mut file = std::fs::File::create(&path)?;
    let resp = http_req::request::Request::new(&uri)
        .header("Cookie", &cookie)
        .send(&mut file)?;
    if !resp.status_code().is_success() {
        drop(file);
        std::fs::remove_file(&path)?;
    }
    Ok(())
}
pub fn get_input(day: u32) -> Result<std::fs::File, Error> {
    let path = generate_path(day);
    let file_in = std::fs::File::open(&path);
    if file_in.is_ok() {
        file_in.map_err(|e| e.into())
    } else {
        download_input(day)?;
        std::fs::File::open(path).map_err(|e| e.into())
    }
}
