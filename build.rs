use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Tell Cargo to re-run this build script if the proto files change
    println!("cargo:rerun-if-changed=saasy-proto/protos/shared/v1/shared.proto");
    println!("cargo:rerun-if-changed=saasy-proto/protos/signal/v1/signal.proto");
    println!("cargo:rerun-if-changed=saasy-proto/protos/sfu/v1/sfu.proto");
    println!("cargo:rerun-if-changed=saasy-proto/protos/listening_engine/v1/listening_engine.proto");
    println!("cargo:rerun-if-changed=saasy-proto/protos/speaking_engine/v1/speaking_engine.proto");
    println!("cargo:rerun-if-changed=saasy-proto/protos/edge/v1/edge.proto");
    println!("cargo:rerun-if-changed=saasy-proto/protos/core/v1/core.proto");

    // Compile all protos with the correct include path
    tonic_build::configure()
        .compile_protos(
            &[
                "saasy-proto/protos/shared/v1/shared.proto",
                "saasy-proto/protos/signal/v1/signal.proto", 
                "saasy-proto/protos/sfu/v1/sfu.proto",
                "saasy-proto/protos/listening_engine/v1/listening_engine.proto",
                "saasy-proto/protos/speaking_engine/v1/speaking_engine.proto",
                "saasy-proto/protos/edge/v1/edge.proto",
                "saasy-proto/protos/core/v1/core.proto",
            ],
            &["saasy-proto"], // This tells protoc where to find imports
        )?;
    
    Ok(())
}
