#[derive(Debug)]
pub enum Error {
    Http(http_req::error::Error),
    Download(http_req::response::StatusCode),
    Io(ex::io::Error),
}

impl From<http_req::error::Error> for Error {
    fn from(e: http_req::error::Error) -> Self {
        Self::Http(e)
    }
}
impl From<ex::io::Error> for Error {
    fn from(e: ex::io::Error) -> Self {
        Self::Io(e)
    }
}
impl From<http_req::response::StatusCode> for Error {
    fn from(e: http_req::response::StatusCode) -> Self {
        Self::Download(e)
    }
}
fn generate_path(day: u32) -> std::path::PathBuf {
    ["input", &day.to_string()].iter().collect()
}
fn download_input(day: u32) -> Result<(), Error> {
    let session = ex::fs::read_to_string("cookie")?;
    let cookie = format!("session={}", session);
    let url = format!("https://adventofcode.com/{}/day/{}/input", 2021, day);
    let url_str: &str = &url;
    let uri = http_req::uri::Uri::try_from(url_str).unwrap();

    let path = generate_path(day);
    let mut file = ex::fs::File::create(&path)?;
    let resp = http_req::request::Request::new(&uri)
        .header("Cookie", &cookie)
        .send(&mut file)?;
    let status = resp.status_code();
    if !status.is_success() {
        drop(file);
        ex::fs::remove_file(&path)?;
        return Err(Error::Download(status));
    }
    Ok(())
}
pub fn get_input(day: u32) -> Result<ex::fs::File, Error> {
    let path = generate_path(day);
    let file_in = ex::fs::File::open(&path);
    if file_in.is_ok() {
        file_in.map_err(|e| e.into())
    } else {
        download_input(day)?;
        ex::fs::File::open(path).map_err(|e| e.into())
    }
}
