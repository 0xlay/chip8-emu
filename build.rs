use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_dir = get_cargo_target_dir();

    link_sdl(target_arch.as_str());
    copy_sdl_to_target(target_dir.as_str(), target_arch.as_str());
}

fn get_cargo_target_dir() -> String {
    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");
    let profile = std::env::var("PROFILE").expect("PROFILE not set");

    let profile_as_osstr = std::ffi::OsString::from(profile);
    let out_dir_path = std::path::PathBuf::from(out_dir);

    let target_dir = out_dir_path
        .ancestors()
        .find(|path| {
            path.file_name()
                .map_or(false, |name| name == profile_as_osstr)
        })
        .expect("Failed to find target directory matching PROFILE");

    target_dir
        .to_str()
        .expect("Path contains invalid UTF-8")
        .to_string()
}

fn get_sdl_dir(target_arch: &str) -> String {
    match target_arch {
        "x86" => "third_party/SDL2/x86",
        "x86_64" => "third_party/SDL2/x64",
        _ => panic!("Unsupported target architecture: {}", target_arch),
    }
    .to_string()
}

fn link_sdl(target_arch: &str) {
    let sdl_dir = get_sdl_dir(target_arch);
    let lib_path = PathBuf::from(sdl_dir.as_str()).join("SDL2.lib");
    if lib_path.exists() {
        println!("cargo:rerun-if-changed=migrations");
        println!("cargo:rustc-link-search=native={}", sdl_dir);
        println!("cargo:rustc-link-lib=static=SDL2");
    } else {
        panic!("SQLite library not found at {}", lib_path.display());
    }
}

fn copy_sdl_to_target(target_dir: &str, target_arch: &str) {
    if !Path::new(target_dir).exists() {
        std::fs::create_dir_all(target_dir).unwrap();
    }

    let lib_path = PathBuf::from(target_dir).join("SDL2.dll");
    let _ = match target_arch {
        "x86" => std::fs::copy("third_party/SDL2/x86/SDL2.dll", lib_path),
        "x86_64" => std::fs::copy("third_party/SDL2/x64/SDL2.dll", lib_path),
        _ => panic!("Unsupported target architecture: {}", target_arch),
    };
}
