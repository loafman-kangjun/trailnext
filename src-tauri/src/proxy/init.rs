// use reqwest::Client;

// #[tokio::main]
// pub async fn init_clash() -> Result<(), reqwest::Error> {
//     let client = Client::new();
//     let res = client
//         .post("http://httpbin.org/post")
//         .body("the exact body that is sent")
//         .send()
//         .await?
//         .text()
//         .await?;
//
//     println!("body: {:?}", res);
//     Ok(())
// }
