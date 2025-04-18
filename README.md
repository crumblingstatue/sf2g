sf2g
=========

Minimal Rust bindings for the Graphics module of [SFML 2](http://www.sfml-dev.org), the Simple and Fast Multimedia Library.
Fork of [rust-sfml](<https://github.com/jeremyletang/rust-sfml.git>).

Platform support
================

| Platform             | Development and testing                                  | CI (basic tests)       |
|----------------------|----------------------------------------------------------|------------------------|
| 🐧 Linux             | ✅ Most tested                                            | ![Status][linux-ci]    |
| 🪟 Windows (MSVC)    | ⚠️ Rarely tested. Looking for testers/maintainers!       | ![Status][win-msvc-ci] |
| 🐧 -> 🪟 (mingw-w64) | ⚠️ Cross-compile from Linux to Windows. Somewhat tested. | ❌                      |
| 🍎 Mac OS X          | ❌ Untested. Looking for testers/maintainers!             | ![Status][macos-ci]    |

[linux-ci]: <>
[win-msvc-ci]: <>
[macos-ci]: <>

Prerequisites
=============

- Rust 1.86 or later
- [CMake](https://cmake.org/), and a C++ toolchain for building SFML
- 🐧 On Linux, you need the following dependencies installed:
   - Window module: `libGL libX11 libXcursor libXrandr`
   - Graphics module: `libfreetype`

License
=======

This software is a binding of the SFML library created by Laurent Gomila, which is provided under the Zlib/png license.

This software is provided under the same license as SFML, the Zlib/png license.
