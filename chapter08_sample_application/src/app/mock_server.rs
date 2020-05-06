// use chapter08_sample_application::infrastructure::{users_api, MockApi, MockContext};
// #[tokio::main]
// async fn main() {
//     let ctx = MockContext::default();
//     let api = MockApi::new(ctx);

//     warp::serve((api))
//         .run(([127, 0, 0, 1], 8080))
//         .await;
// }
use anyhow::Result;

use chapter08_sample_application::domain::{User, MailAddress, Name};

fn main() {
    println!("Hello, World");
}