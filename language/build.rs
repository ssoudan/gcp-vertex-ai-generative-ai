//! Build script for the language crate.
use std::io::Result;

fn main() -> Result<()> {
    tonic_prost_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .build_server(false)
        .compile_protos(
            &[
                "protos/google/ai/generativelanguage/v1/generative_service.proto",
                "protos/google/ai/generativelanguage/v1/model_service.proto",
            ],
            &["protos/"],
        )?;
    Ok(())
}
