fn main() {
    println!("cargo:rerun-if-changed=vendor/MT/dSFMT.c");
    println!("cargo:rerun-if-changed=vendor/MT/dSFMT.h");
    cc::Build::new()
        .file("vendor/MT/dSFMT.c")
        .flag("-finline-functions")
        .flag("-fomit-frame-pointer")
        .flag("-DNDEBUG")
        .flag("-fno-strict-aliasing")
        .flag("--param")
        .flag("max-inline-insns-single=1800")
        .opt_level(3)
        .flag("-msse2")
        .flag("-flto=thin")
        .flag("-DHAVE_SSE2")
        .compile("dSFMT");
}
