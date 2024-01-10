fn main() -> miette::Result<()> {
    let path_src = std::path::PathBuf::from("src"); // include path
    let path_cxx = std::path::PathBuf::from("cxx"); // include path

    let mut b = autocxx_build::Builder::new("src/ffi.rs", [&path_src, &path_cxx])
        .extra_clang_args(&["-std=c++17"])
        .build()?;
    b.flag_if_supported("-std=c++17")
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-#pragma-messages")
        .compile("autocxx-demo"); // arbitrary library name, pick anything
    println!("cargo:rerun-if-changed=src/ffi.rs");

    Ok(())
}
