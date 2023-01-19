use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut builder = cc::Build::new();
    let target = env::var("TARGET")?;
    let include_paths = String::from_utf8(
        cc::Build::new()
            .get_compiler()
            .to_command()
            .arg("-E")
            .arg("-Wp,-v")
            .arg("-xc")
            .arg("/dev/null")
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to run the compiler to get paths")
            .wait_with_output()
            .expect("Failed to run the compiler to get paths")
            .stderr,
    )
    .unwrap()
    .lines()
    .filter_map(|line| line.strip_prefix(" "))
    .map(|path| format!("-isystem{}", path))
    .collect::<Vec<_>>();

    let builder = builder
        .warnings(false)
        .flag("-DLFS_NO_MALLOC")
        .flag("-DLFS_NO_DEBUG")
        .flag("-DLFS_NO_WARN")
        .flag("-DLFS_NO_ERROR")
        .file("littlefs/lfs.c")
        .file("littlefs/lfs_util.c")
        .file("string.c");

    #[cfg(not(feature = "assertions"))]
    let builder = builder.flag("-DLFS_NO_ASSERT");

    #[cfg(feature = "trace")]
    let builder = builder.flag("-DLFS_YES_TRACE");

    builder.compile("lfs-sys");

    let bindings = bindgen::Builder::default()
        .header("littlefs/lfs.h")
        .clang_arg(format!("--target={}", target))
        .clang_args(&include_paths)
        .use_core()
        .ctypes_prefix("cty")
        .rustfmt_bindings(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    Ok(())
}
