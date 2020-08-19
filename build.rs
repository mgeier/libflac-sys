fn main() {
    let mut flac_config = cmake::Config::new("flac");
    flac_config
        .define("BUILD_CXXLIBS", "OFF")
        .define("BUILD_PROGRAMS", "OFF")
        .define("BUILD_EXAMPLES", "OFF")
        .define("BUILD_DOCS", "OFF")
        .define("INSTALL_MANPAGES", "OFF")
        .define("INSTALL_PKGCONFIG_MODULES", "OFF")
        .define("INSTALL_CMAKE_CONFIG_MODULE", "OFF");

    if cfg!(feature = "ogg") {
        let ogg_path = cmake::Config::new("ogg")
            .define("BUILD_SHARED_LIBS", "OFF")
            .define("INSTALL_DOCS", "OFF")
            .define("INSTALL_PKG_CONFIG_MODULE", "OFF")
            .define("INSTALL_CMAKE_PACKAGE_MODULE", "OFF")
            .build();
        println!("cargo:rustc-link-search=native={}/lib", ogg_path.display());
        println!("cargo:rustc-link-lib=static=ogg");
        flac_config.define("CMAKE_PREFIX_PATH", ogg_path);
        flac_config.define("WITH_OGG", "ON");
    } else {
        flac_config.define("WITH_OGG", "OFF");
    };

    let flac_path = flac_config.build();
    println!("cargo:rustc-link-search=native={}/lib", flac_path.display());
    println!("cargo:rustc-link-lib=static=FLAC");
}
