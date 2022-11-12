#[cfg(not(feature = "build-flac"))]
fn main() {
    println!("cargo:rustc-link-lib=FLAC");
}

#[cfg(feature = "build-flac")]
fn main() {
    let mut flac_config = cmake::Config::new("flac");
    flac_config
        .define("BUILD_CXXLIBS", "OFF")
        .define("BUILD_PROGRAMS", "OFF")
        .define("BUILD_EXAMPLES", "OFF")
        .define("BUILD_TESTING", "OFF")
        .define("BUILD_DOCS", "OFF")
        .define("INSTALL_MANPAGES", "OFF")
        .define("INSTALL_PKGCONFIG_MODULES", "OFF")
        .define("INSTALL_CMAKE_CONFIG_MODULE", "OFF");

    if cfg!(feature = "build-ogg") {
        let ogg_path = cmake::Config::new("ogg")
            .define("BUILD_SHARED_LIBS", "OFF")
            .define("INSTALL_DOCS", "OFF")
            .define("INSTALL_PKG_CONFIG_MODULE", "OFF")
            .define("INSTALL_CMAKE_PACKAGE_MODULE", "OFF")
            .build();
        println!("cargo:rustc-link-search=native={}/lib", ogg_path.display());
        // This path is used on 64 bit Fedora 33:
        println!(
            "cargo:rustc-link-search=native={}/lib64",
            ogg_path.display()
        );
        println!("cargo:rustc-link-lib=static=ogg");

        flac_config.define("CMAKE_PREFIX_PATH", ogg_path);
        flac_config.define("WITH_OGG", "ON");
    } else {
        flac_config.define("WITH_OGG", "OFF");
    };

    let flac_path = flac_config.build_target("FLAC").build();
    if std::env::var("TARGET")
        .expect("TARGET is defined by rustc")
        .contains("msvc")
    {
        println!(
            "cargo:rustc-link-search=native={}/build/src/libFLAC/{}",
            flac_path.display(),
            flac_config.get_profile()
        );
    } else {
        println!(
            "cargo:rustc-link-search=native={}/build/src/libFLAC",
            flac_path.display()
        );
    }
    println!("cargo:rustc-link-lib=static=FLAC");
}
