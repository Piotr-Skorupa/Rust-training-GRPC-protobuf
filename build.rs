fn main() {
    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .out_dir("tonic_generated")  // you can change the generated code's location
        .compile(
            &["proto/sensor.proto"],
            &["proto"], // specify the root location to search proto dependencies
        ).unwrap();
}