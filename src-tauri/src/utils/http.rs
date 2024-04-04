use tauri_plugin_http::reqwest;

pub async fn get() {
    let resp = reqwest::get("https://httpbin.org/ip").await.expect("REASON").text().await;
    println!("{resp:#?}");
}
