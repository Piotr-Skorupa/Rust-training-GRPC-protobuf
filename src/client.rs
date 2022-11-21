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
        print!(concat!("SENSOR CLIENT\n[temperature;pressure;humidity] - type data to send separeted by semicolons\n",
            "get - to get last 5 data sent packages\n[q] - to exit\n"));
        match std::io::stdin().read_line(&mut line) {
            Ok(_n) => {
                line = line.trim().to_owned();
                line = line.replace(" ", "");
                line = line.replace(",", ".");
                if line == String::from("q") {
                    println!("Exit");
                } else if line == String::from("get") {
                    let response = client.get_last_five_data_packages(tonic::Request::<()>::new(())).await?;
                    println!("Last 5 data packages: {:?}", response.get_ref());
                } else {
                    println!("Processing line: {}", line);
                    let splitted = line.split(";").collect::<Vec<&str>>();
                    println!("{:?}", splitted);

                    let Ok(temperature) = splitted[0].parse::<f32>() else {
                        eprintln!("Temperature parsing error. Please type correct value!");
                        continue;
                    };

                    let Ok(pressure) = splitted[0].parse::<i32>() else {
                        eprintln!("Pressure parsing error. Please type correct value!");
                        continue;
                    };

                    let Ok(humidity) = splitted[0].parse::<i32>() else {
                        eprintln!("Humidity parsing error. Please type correct value!");
                        continue;
                    };

                    let request = tonic::Request::new(SensorData {
                        temperature: temperature,
                        pressure: pressure,
                        humidity: humidity
                    });

                    println!("Trying to send request: {:?}", request.get_ref());
                    let response = client.send_data(request).await?;
                    println!("Response: {:?}", response);
                }
            },
            Err(_error) => {
                exit(ABORT_CODE);
            }
        }
    }

    Ok(())
}
