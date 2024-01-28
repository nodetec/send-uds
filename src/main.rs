// use tokio::net::UnixStream;
// use tokio::io::AsyncWriteExt;
//
// #[tokio::main]
// async fn main() {
//     let path = "/tmp/rust-uds.sock";
//
//     match UnixStream::connect(path).await {
//         Ok(mut stream) => {
//             let message = "Hello from the client";
//             if let Err(e) = stream.write_all(message.as_bytes()).await {
//                 eprintln!("Failed to send message: {}", e);
//             } else {
//                 println!("Message sent successfully");
//             }
//         }
//         Err(e) => {
//             eprintln!("Failed to connect to the socket: {}", e);
//         }
//     }
// }
//

use serde_json::{json, to_string};
use tokio::net::UnixStream;
use tokio::io::AsyncWriteExt;

#[tokio::main]
async fn main() {
    let json_obj = json!({
        "name": "John Doe",
        "age": 30
    });

    let json_str = to_string(&json_obj).expect("Failed to serialize JSON");

    // Replace with your Unix Domain Socket path
    let path = "/tmp/rust-uds.potato";
    let mut stream = UnixStream::connect(path).await.expect("Failed to connect to socket");

    stream.write_all(json_str.as_bytes()).await.expect("Failed to write to socket");
}
