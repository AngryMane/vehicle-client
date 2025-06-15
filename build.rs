fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile(
            &["external/vehicle-protocol/proto/vehicle-shadow/signal.proto"],
            &["external/vehicle-protocol/proto"],
        )?;
    Ok(())
}
