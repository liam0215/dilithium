use bindgen;

fn main() {
    let src = [
        "ref/randombytes.c",
        "ref/poly.c",
        "ref/polyvec.c",
        "ref/reduce.c",
        "ref/sign.c",
        "ref/ntt.c",
        "ref/packing.c",
        "ref/rounding.c",
        "ref/fips202.c",
    ];
    cc::Build::new()
        .files(src.iter())
        .include("ref/")
        .flag("-g")
        .flag("-Wall")
        .flag("-Wextra")
        .flag("-O3")
        .flag("-fomit-frame-pointer")
        .flag("-fPIC")
        .static_flag(true)
        .compile("dilithium");

    let bindings = bindgen::Builder::default()
        .header("ref/api.h")
        .header("ref/randombytes.h")
        .allowlist_var("CRYPTO_.*")
        .allowlist_function("crypto_sign.*")
        .allowlist_function("randombytes")
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");
    bindings
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");

    println!("cargo:rerun-if-changed=build.rs");
}
