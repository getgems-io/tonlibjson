use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env::set_var("PROTOC", protobuf_src::protoc());

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .type_attribute("TvmEmulatorRunGetMethodResponse", "#[derive(serde::Deserialize, serde::Serialize)]")
        .file_descriptor_set_path(out_dir.join("ton_descriptor.bin"))
        .compile(&["proto/ton.proto"], &["proto"])?;

    Ok(())
}
