
pub fn init_clash() {
    let client = reqwest::Client::new();
    let body = client.get("http://httpbin.org/get")
        .text();
    println!("{}", body)

}
