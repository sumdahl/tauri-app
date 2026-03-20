fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();

    let default_backend_url = if target_os == "android" {
        "http://10.200.230.210:3000"
    } else {
        "http://127.0.0.1:3000"
    };

    let backend_url =
        std::env::var("PUBLIC_BACKEND_URL").unwrap_or_else(|_| default_backend_url.to_string());

    println!("cargo:rustc-env=BACKEND_URL={}", backend_url);
    println!("cargo:rerun-if-env-changed=PUBLIC_BACKEND_URL");

    tauri_build::build()
}
