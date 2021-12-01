fn generate_path(day: u32) -> std::path::PathBuf {
    ["input", &day.to_string()].iter().collect()
}
fn download_input(day: u32) -> Result<(), ureq::Error> {
    let cookie = std::fs::read_to_string("cookie")?;
    let resp = ureq::get(&format!("https://adventofcode.com/2021/day/{}/input", day))
        .set("session", &cookie)
        .call()?
        .into_string()?;
    std::fs::write(generate_path(day), resp)?;
    Ok(())
}
pub fn get_input(day: u32) -> Result<std::fs::File, ureq::Error> {
    let path = generate_path(day);
    let file_in = std::fs::File::open(&path);
    if file_in.is_ok() {
        file_in.map_err(|e| e.into())
    } else {
        download_input(day)?;
        std::fs::File::open(path).map_err(|e| e.into())
    }
}
