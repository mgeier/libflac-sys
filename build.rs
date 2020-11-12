#[cfg(not(feature = "build-flac"))]
fn main() {
    println!("cargo:rustc-link-lib=FLAC");
}

#[cfg(feature = "build-flac")]
fn main() {
    let mut flac_config = cmake::Config::new("flac");
    flac_config
        .define("BUILD_CXXLIBS", "OFF")
        .define("BUILD_EXAMPLES", "OFF")
        .define("BUILD_DOXYGEN", "OFF")
        .define("BUILD_TESTING", "OFF");

    if cfg!(feature = "build-ogg") {
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

    let flac_path = flac_config.build_target("FLAC").build();
    if std::env::var("TARGET")
        .expect("TARGET is defined by rustc")
        .contains("msvc")
    {
        // cmake automatically picks the build profile, and MSVC binary location depends thereon;
        // this code is borrowed from https://docs.rs/cmake/0.1.44/src/cmake/lib.rs.html#498,
        // but see https://github.com/alexcrichton/cmake-rs/pull/105 for a replacement
        let profile = {
            // Automatically set the `CMAKE_BUILD_TYPE` if the user did not
            // specify one.

            // Determine Rust's profile, optimization level, and debug info:
            #[derive(PartialEq)]
            enum RustProfile {
                Debug,
                Release,
            }
            #[derive(PartialEq, Debug)]
            enum OptLevel {
                Debug,
                Release,
                Size,
            }

            let rust_profile =
                match &std::env::var("PROFILE").expect("PROFILE is defined by rustc")[..] {
                    "debug" => RustProfile::Debug,
                    "release" | "bench" => RustProfile::Release,
                    unknown => {
                        eprintln!(
                            "Warning: unknown Rust profile={}; defaulting to a release build.",
                            unknown
                        );
                        RustProfile::Release
                    }
                };

            let opt_level =
                match &std::env::var("OPT_LEVEL").expect("OPT_LEVEL is defined by rustc")[..] {
                    "0" => OptLevel::Debug,
                    "1" | "2" | "3" => OptLevel::Release,
                    "s" | "z" => OptLevel::Size,
                    unknown => {
                        let default_opt_level = match rust_profile {
                            RustProfile::Debug => OptLevel::Debug,
                            RustProfile::Release => OptLevel::Release,
                        };
                        eprintln!(
                            "Warning: unknown opt-level={}; defaulting to a {:?} build.",
                            unknown, default_opt_level
                        );
                        default_opt_level
                    }
                };

            let debug_info = match &std::env::var("DEBUG").expect("DEBUG is defined by rustc")[..] {
                "false" => false,
                "true" => true,
                unknown => {
                    eprintln!("Warning: unknown debug={}; defaulting to `true`.", unknown);
                    true
                }
            };

            match (opt_level, debug_info) {
                (OptLevel::Debug, _) => "Debug",
                (OptLevel::Release, false) => "Release",
                (OptLevel::Release, true) => "RelWithDebInfo",
                (OptLevel::Size, _) => "MinSizeRel",
            }
        };

        println!(
            "cargo:rustc-link-search=native={}/build/src/libFLAC/{}",
            flac_path.display(),
            profile
        );
    } else {
        println!(
            "cargo:rustc-link-search=native={}/build/src/libFLAC",
            flac_path.display()
        );
    }
    println!("cargo:rustc-link-lib=static=FLAC");
}
