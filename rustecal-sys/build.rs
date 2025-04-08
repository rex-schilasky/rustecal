use std::{env, path::PathBuf};

fn main() {
    // Get ECAL_HOME from environment
    let ecal_home = env::var("ECAL_HOME").expect("ECAL_HOME environment variable must be set");

    let include_path = format!("{}/include", ecal_home);
    let lib_path = format!("{}/lib", ecal_home);

    // Tell cargo to rebuild if wrapper.h changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // Generate bindings using bindgen
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", include_path))
        .allowlist_function("eCAL_.*")
        .allowlist_type("eCAL_.*")
        .allowlist_var("eCAL_.*")
        .layout_tests(false)
        .generate_comments(true)
        .derive_default(true)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Link with ecal_core_c.lib
    println!("cargo:rustc-link-lib=static=ecal_core_c");
    println!("cargo:rustc-link-search=native={}", lib_path);
}
