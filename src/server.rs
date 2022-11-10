use sensor::{ SensorData, Status };
use sensor::data_handler_server::{ DataHandler, DataHandlerServer };

pub mod sensor {
    include!("../tonic_generated/sensor.rs");
}

#[derive(Default)]
pub struct RealDataHandler {}

#[tonic::async_trait]
impl DataHandler for RealDataHandler {
    async fn send_data(
        &self,
        request: tonic::Request<SensorData>,
    ) -> Result<tonic::Response<Status>, tonic::Status> {
        println!("Got a request {:?} from {:?}", request.get_ref(), request.remote_addr());

        let reply = Status{ ok: true };
        Ok(tonic::Response::new(reply))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let handler = RealDataHandler::default();

    println!("GreeterServer listening on {}", addr);

    tonic::transport::Server::builder()
        .add_service(DataHandlerServer::new(handler))
        .serve(addr)
        .await?;

    Ok(())
}
