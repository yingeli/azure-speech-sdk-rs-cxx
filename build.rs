use std::{env, path::PathBuf};

fn main() {
    let speechsdk_root = env::var_os("SPEECHSDK_ROOT")
        .map(PathBuf::from)
        .or_else(|| {
            env::var_os("HOME")
                .map(PathBuf::from)
                .map(|home| home.join("speechsdk"))
        })
        .unwrap_or_else(|| PathBuf::from("speechsdk"));

    let include_cxx_api = speechsdk_root.join("include").join("cxx_api");
    let include_c_api = speechsdk_root.join("include").join("c_api");

    cxx_build::bridge("src/sys/translation/speech_translation_config.rs")
        .include(include_cxx_api)
        .include(include_c_api)
        .std("c++20")
        .compile("azure-speech-sdk");

    println!("cargo:rerun-if-changed=src/sys");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=SPEECHSDK_ROOT");
    println!("cargo:rerun-if-env-changed=HOME");
}
