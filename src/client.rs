use sensor::SensorData;
use sensor::data_handler_client::DataHandlerClient;
use std::{process::exit};

const ABORT_CODE: i32 = 6;

pub mod sensor {
    include!("../tonic_generated/sensor.rs");
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = DataHandlerClient::connect("http://[::1]:50051").await?;

    let mut line: String = String::new();
    while line != String::from("q") {
        line.clear();
        print!("SENSOR CLIENT\n[temperature;pressure;humidity] - type data to send separeted by semicolons\n[q] - to exit\n");
        match std::io::stdin().read_line(&mut line) {
            Ok(_n) => {
                line = line.trim().to_owned();
                line = line.replace(" ", "");
                line = line.replace(",", ".");
                if line == String::from("q") {
                    println!("Exit");
                    continue;
                }

                println!("Processing line: {}", line);
                let splitted = line.split(";").collect::<Vec<&str>>();
                println!("{:?}", splitted);

                let request = tonic::Request::new(SensorData {
                    temperature: splitted[0].parse::<f32>().unwrap_or(0.0),
                    pressure: splitted[1].parse::<i32>().unwrap_or(0),
                    humidity: splitted[2].parse::<i32>().unwrap_or(0)
                });

                println!("Trying to send request: {:?}", request.get_ref());
                let response = client.send_data(request).await?;
                println!("Response: {:?}", response);
            },
            Err(_error) => {
                exit(ABORT_CODE);
            }
        }
    }

    Ok(())
}
