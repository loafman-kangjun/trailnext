use tauri_plugin_http::reqwest;

pub fn text() {
    let body = reqwest::blocking::get("https://www.rust-lang.org")?
        .text()?;
    println!("body = {:?}", body);
}

pub fn download() {
    let body = reqwest::blocking::get("https://www.rust-lang.org")?
        .text()?;
    println!("body = {:?}", body);
}
