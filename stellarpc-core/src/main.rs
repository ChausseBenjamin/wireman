use http::Uri;
use std::sync::mpsc;
use stellarpc_core::client::async_call_unary;
use stellarpc_core::client::create_runtime;
use stellarpc_core::ProtoDescriptor;
use stellarpc_core::Result;

fn main() -> Result<()> {
    _test_async()?;

    Ok(())
}
fn _test_async() -> Result<()> {
    // Create a channel for the event stream
    let (tx, rx) = mpsc::channel::<String>();

    let desc = ProtoDescriptor::new(
        vec!["/Users/philippreiter/Rust/stellarpc/test_utils"],
        vec!["grpc_simple/debugger.proto"],
    )?;
    // Services and methods
    let service = &desc.get_services()[0];
    let method = &desc.get_methods(&service)[1];

    // Request
    let mut req = desc.get_request(&method);
    req.message_mut().apply_template();
    req.set_address("http://localhost:50051");
    req.insert_metadata("key", "value")?;

    // Call grpc
    let rt = create_runtime()?;
    let _ = rt.spawn(async move {
        let uri = Uri::try_from(req.address()).unwrap();
        let resp = async_call_unary(uri, &req).await;
        println!("Response {:?}", resp);
        if let Err(err) = tx.send(String::from("Hello!")) {
            eprintln!("Error sending event: {:?}", err);
        }
    });

    // Process events from the stream
    loop {
        if let Ok(code) = rx.recv() {
            println!("Received event: {:?}", code,);
            break;
        }
    }

    // Shut down runtime
    rt.shutdown_background();

    return Ok(());
}

fn _test_oneof() -> Result<()> {
    let desc = ProtoDescriptor::new(
        vec!["/Users/philippreiter/Rust/stellarpc/stellarpc-core"],
        vec!["test_files/oneof.proto"],
    )?;
    let services = desc.get_services();
    for service in services.clone() {
        println!("{:?}", service.name());
    }
    let service = &services[0];
    let method = &desc.get_methods(&service)[0];
    let mut req = desc.get_request(&method);

    req.message_mut().apply_template();
    println!("{:?}", req.message().to_json());
    return Ok(());
}
