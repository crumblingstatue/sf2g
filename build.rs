use std::env;

fn static_link_windows() {
    println!("cargo:rustc-link-lib=dylib=winmm");
    println!("cargo:rustc-link-lib=dylib=user32");
    println!("cargo:rustc-link-lib=dylib=opengl32");
    println!("cargo:rustc-link-lib=dylib=gdi32");
}

fn static_link_linux() {
    println!("cargo:rustc-link-lib=dylib=udev");
    println!("cargo:rustc-link-lib=dylib=GL");
    println!("cargo:rustc-link-lib=dylib=X11");
    println!("cargo:rustc-link-lib=dylib=Xcursor");
    println!("cargo:rustc-link-lib=dylib=Xrandr");
}

enum WinEnv {
    Gnu,
    Msvc,
}

impl WinEnv {
    fn get() -> Option<Self> {
        match env::var("CARGO_CFG_TARGET_ENV").as_deref() {
            Ok("gnu") => Some(Self::Gnu),
            Ok("msvc") => Some(Self::Msvc),
            _ => None,
        }
    }
}

fn main() {
    println!("cargo:rerun-if-changed=CSFML");
    println!("cargo:rerun-if-changed=SFML");
    let mut cmake = cmake::Config::new("SFML");
    let win_env = WinEnv::get();
    // Due to complications with static linking of MSVC runtime (debug version),
    // we cannot support debug builds of SFML.
    cmake.profile("Release");
    cmake
        //.define("CMAKE_FIND_DEBUG_MODE", "TRUE") // When debugging not being able to find a library
        .define("BUILD_SHARED_LIBS", "FALSE")
        .define("SFML_INSTALL_PKGCONFIG_FILES", "FALSE")
        // Disable "install" step
        .no_build_target(true);
    let cmake_build_path = cmake.build();
    let mut build = cc::Build::new();
    build
        .cpp(true)
        .flag_if_supported("--std=c++17")
        .define("CSFML_SYSTEM_EXPORTS", None)
        .define("CSFML_WINDOW_EXPORTS", None)
        .define("CSFML_GRAPHICS_EXPORTS", None)
        .define("SFML_STATIC", None)
        .include("CSFML/src/")
        .include("SFML/include");
    build.files(
        [
            "CSFML/src/System/Clock.cpp",
            "CSFML/src/System/Sleep.cpp",
            "CSFML/src/System/SfString.cpp",
            "CSFML/src/System/SfStdString.cpp",
            "CSFML/src/System/SfStdVector.cpp",
            "CSFML/src/Window/Cursor.cpp",
            "CSFML/src/Window/Joystick.cpp",
            "CSFML/src/Window/Keyboard.cpp",
            "CSFML/src/Window/Mouse.cpp",
            "CSFML/src/Window/Sensor.cpp",
            "CSFML/src/Window/Touch.cpp",
            "CSFML/src/Window/VideoMode.cpp",
            "CSFML/src/Window/Window.cpp",
            "CSFML/src/Window/Context.cpp",
            "CSFML/src/Graphics/CircleShape.cpp",
            "CSFML/src/Graphics/ConvexShape.cpp",
            "CSFML/src/Graphics/Font.cpp",
            "CSFML/src/Graphics/RectangleShape.cpp",
            "CSFML/src/Graphics/RenderTexture.cpp",
            "CSFML/src/Graphics/RenderWindow.cpp",
            "CSFML/src/Graphics/Shader.cpp",
            "CSFML/src/Graphics/Sprite.cpp",
            "CSFML/src/Graphics/Texture.cpp",
            "CSFML/src/Graphics/Transform.cpp",
            "CSFML/src/Graphics/VertexBuffer.cpp",
            "CSFML/src/Graphics/View.cpp",
        ]
        .iter(),
    );
    build.compile("rcsfml");
    // Need to probe Cargo's env as build.rs uses the default toolchain to
    // run the build meaning that #[cfg(..)]'s won't work
    let is_windows = env::var("CARGO_CFG_WINDOWS").is_ok();
    let is_unix = env::var("CARGO_CFG_UNIX").is_ok();
    let is_linux = env::var("CARGO_CFG_TARGET_OS")
        .map(|os| os == "linux")
        .unwrap_or(false);
    let is_macos = env::var("CARGO_CFG_TARGET_OS")
        .map(|os| os == "macos")
        .unwrap_or(false);
    if is_macos {
        link_mac_os_frameworks();
    }
    let link_search = if matches!(win_env, Some(WinEnv::Msvc)) {
        "build/lib/Release"
    } else {
        "build/lib"
    };
    let build_lib_path = cmake_build_path.join(link_search);
    println!(
        "cargo:rustc-link-search=native={}",
        build_lib_path.display()
    );
    println!("cargo:rustc-link-lib=static=rcsfml");
    println!("cargo:rustc-link-lib=static=freetype2");
    println!("cargo:rustc-link-lib=static=png");
    println!("cargo:rustc-link-lib=static=z");
    link_sfml_subsystem("system");
    if is_unix && is_linux {
        static_link_linux();
    } else if is_windows {
        match win_env {
            Some(_) => {
                static_link_windows();
            }
            None => {
                panic!("Failed to determine windows environment (MSVC/Mingw)");
            }
        }
    } else if is_macos {
        // SFML contains Objective-C source files on OSX
        // https://github.com/SFML/SFML/issues/2920
        println!("cargo::rustc-link-arg=-ObjC");
    } else {
        panic!("Uhhh... Can't determine your environment. Sorry.");
    }

    link_sfml_subsystem("window");
    link_sfml_subsystem("graphics");
}

fn link_sfml_subsystem(name: &str) {
    println!("cargo:rustc-link-lib=static=sfml-{name}-s");
}

fn link_mac_os_frameworks() {
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
    println!("cargo:rustc-link-lib=framework=CoreGraphics");
    println!("cargo:rustc-link-lib=framework=IOKit");
    println!("cargo:rustc-link-lib=framework=Carbon");
    println!("cargo:rustc-link-lib=framework=AppKit");
    println!("cargo:rustc-link-lib=framework=OpenAL");
}
