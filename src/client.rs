use sensor::SensorData;
use sensor::data_handler_client::DataHandlerClient;

pub mod sensor {
    include!("../tonic_generated/sensor.rs");
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DataHandlerClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(SensorData {
        temperature: 22.5,
        pressure: 1094,
        humidity: 68
    });

    println!("Trying to send request: {:?}", request.get_ref());
    let response = client.send_data(request).await?;
    println!("Response: {:?}", response);

    Ok(())
}
