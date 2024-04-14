use reqwest;

pub fn text() {
    let body = reqwest::blocking::get("https://www.rust-lang.org").expect("REASON");
    println!("body = {:?}", body);
}

pub fn download() {
    let body = reqwest::blocking::get("https://www.rust-lang.org").expect("REASON");
    println!("body = {:?}", body);
}
