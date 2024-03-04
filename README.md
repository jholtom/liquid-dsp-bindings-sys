# liquid-dsp-bindings-sys

This provides (unsafe) rust bindings to the [liquid-dsp](https://github.com/jgaeddert/liquid-dsp) digital signal processing library.

This was primarily designed to work with Linux, but it *does* work with Windows (albeit it's a little tricky to get working).
I don't have a Mac so I am unable to verify any functionality whatsoever on MacOS.

Unfortunately, this crate does not build liquid-dsp for you, as that's a whole (platform-specific) challenge in itself.
## Usage
This crate can utilize liquid-dsp in 2 ways:
1. Build and install liquid-dsp into your system normally by following the official [liquid-dsp installation guide](https://liquidsdr.org/doc/installation/).
2. Build liquid-dsp, clone this repo, create a directory named `liquid`, and copy your compiled `libliquid.a` and `liquid.h` header files into the directory.

That should be it! (*For Linux users, anyway..*)

## For Windows users:
liquid-dsp doesn't *really* have support for windows, because the MSVC compiler doesn't support all of the C99 features that liquid-dsp requires.
You can get around this by using MingW64 to build liquid-dsp. Unfortunately, the official installation guide with autotools didn't work for me,
so I ported liquid-dsp to work with CMake. If you're interested, check out my [pull request](https://github.com/jgaeddert/liquid-dsp/pull/353).
Also, things don't seem to work right if you're using the (default) `stable-x86_64-pc-windows-msvc` rust toolchain. Use the `stable-x86_64-pc-windows-gnu` toolchain instead.

If you're using CMake to build liquid-dsp on Windows, you should run cmake with the `-DCMAKE_INSTALL_PREFIX=/ucrt64` argument so that rust can find find the liquid-dsp library and headers.
