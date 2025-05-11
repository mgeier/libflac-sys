#[cfg(not(feature = "build-flac"))]
fn main() {
    println!("cargo:rustc-link-lib=FLAC");
}

fn detect_ndk_root() -> Option<std::path::PathBuf> {
    if let Ok(cc) = std::env::var("CC") {
        return std::path::Path::new(&cc)
            .ancestors()
            .nth(5)
            .map(std::path::PathBuf::from);
    }
    std::env::var("ANDROID_NDK_HOME")
        .or_else(|_| std::env::var("NDK_HOME"))
        .ok()
        .map(std::path::PathBuf::from)
}
fn apply_android_toolchain(cfg: &mut cmake::Config) {
    let target = std::env::var("TARGET").unwrap();
    if !target.contains("android") {
        return;
    }

    if let Some(ndk) = detect_ndk_root() {
        cfg.define(
            "CMAKE_TOOLCHAIN_FILE",
            ndk.join("build/cmake/android.toolchain.cmake"),
        );
    } else {
        eprintln!("cargo:warning=NDK not found; relying on .cargo/config or CMake defaults");
    }

    let abi = match target.split('-').next().unwrap() {
        "aarch64" => "arm64-v8a",
        "armv7" | "arm" => "armeabi-v7a",
        "x86_64" => "x86_64",
        "i686" => "x86",
        other => panic!("Unsupported Android arch: {}", other),
    };
    cfg.define("ANDROID_ABI", abi);

    let api = target
        .split("android")
        .nth(1)
        .and_then(|s| {
            s.chars()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<u32>()
                .ok()
        })
        .unwrap_or(21);
    cfg.define("ANDROID_PLATFORM", format!("android-{api}"));

    if std::env::var("HOST")
        .unwrap_or_default()
        .contains("windows")
    {
        cfg.generator("Ninja");
    }
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
    
    apply_android_toolchain(&mut flac_config);
    
    if cfg!(feature = "build-ogg") {
        let mut ogg_config = cmake::Config::new("ogg");
        cmake::Config::new("ogg")
            .define("BUILD_SHARED_LIBS", "OFF")
            .define("INSTALL_DOCS", "OFF")
            .define("INSTALL_PKG_CONFIG_MODULE", "OFF")
            .define("INSTALL_CMAKE_PACKAGE_MODULE", "OFF")
            // This is needed with CMake 4.x (until a new OGG release):
            .define("CMAKE_POLICY_VERSION_MINIMUM", "3.5");
        
        apply_android_toolchain(&mut ogg_config);
        
        let ogg_path = ogg_config.build();
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
    //disable bindgen layout-tests for x86-32
    println!("cargo:rustc-cfg=bindgen_disable_layout_tests");
}
