use std::sync::Mutex;
use data_storage::sensor::{ SensorData, Status, SensorDataPackages };
use data_storage::sensor::data_handler_server::{ DataHandler, DataHandlerServer };

mod data_storage;

#[derive(Default)]
pub struct RealDataHandler {
    data_storage_: Mutex<data_storage::DataStorage>
}

#[tonic::async_trait]
impl DataHandler for RealDataHandler {
    async fn send_data(
        &self,
        request: tonic::Request<SensorData>,
    ) -> Result<tonic::Response<Status>, tonic::Status> {
        println!("Got a request {:?} from {:?}", request.get_ref(), request.remote_addr());

        let reply = Status{ ok: true, description: String::from("Ok") };
        let mut lock = self.data_storage_.lock().unwrap();
        (*lock).add(request.get_ref().clone());
        Ok(tonic::Response::new(reply))
    }

    async fn get_last_five_data_packages(
        &self,
        _request: tonic::Request<()>,
    ) -> Result<tonic::Response<SensorDataPackages>, tonic::Status> {
        let lock = self.data_storage_.lock().unwrap();
        Ok(tonic::Response::new(SensorDataPackages { packages: (*lock.get_packages(5)).to_vec() }))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let handler = RealDataHandler::default();

    println!("RealDataHandler listening on {}", addr);

    tonic::transport::Server::builder()
        .add_service(DataHandlerServer::new(handler))
        .serve(addr)
        .await?;

    Ok(())
}
