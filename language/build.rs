//! Build script for the language crate.
use std::io::Result;

fn main() -> Result<()> {
    tonic_build::configure()
        .protoc_arg("--experimental_allow_proto3_optional")
        .build_server(false)
        .compile(
            &[
                "protos/google/ai/generativelanguage/v1beta2/discuss_service.proto",
                "protos/google/ai/generativelanguage/v1beta2/model_service.proto",
                "protos/google/ai/generativelanguage/v1beta2/text_service.proto",
            ],
            &["protos/"],
        )?;
    Ok(())
}
