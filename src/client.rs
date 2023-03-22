use hello_world::greeter_client::GreeterClient;
use hello_world::HelloRequest;
use std::env;

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = GreeterClient::connect("http://[::1]:50051").await?;
    let args: Vec<String> = env::args().collect();
    let name = args.get(1).map(|s| s.as_ref()).unwrap_or("World");

    let request = tonic::Request::new(HelloRequest {
        name: name.to_string(),
    });

    // let request = tonic::Request::new(HelloRequest {
    //     name: "Jack".into(),
    // });

    let response = client.say_hello(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}