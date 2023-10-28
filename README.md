# KKB

> ### It's just a fact: _**<ins>K</ins>ool <ins>K</ins>ids have nice <ins>B</ins>ackgrounds**_.

<p align="center">
  <img src="https://github.com/DevinLeamy/kkb/assets/45083086/28f492a6-0ffe-4dcf-8c94-b80bf7abd2a9" width="300" height="300">
</p>

### Install

```bash
cargo install kkb
```

Ensure you have OpenCV installed. It can be finiky - _I wish it wasn't_.

### Usage

Make your background beautiful.

```bash
kkb
```

Describe your work of art.

```bash
kkb "Hercules riding a motorcycle over a rainbow, with a jetpack"
```

### Options

- `-o <path>`: Output image to path.

### Tech

-   Image generation: [OpenAI's DALL-E 3](https://openai.com/dall-e-3)
-   Image super resolution: [OpenCV](https://github.com/twistedfall/opencv-rust)

### FAQ

-   [Macos] Can't install OpenCV? Try `brew install opencv`.

    1.  You may need to [reinstall homebrew](https://github.com/Genymobile/scrcpy/issues/2128#issuecomment-958046872) if you recently updated your OS.
    2.  You may need to set this library path:

    ```bash
    export DYLD_FALLBACK_LIBRARY_PATH="$(xcode-select --print-path)/Toolchains/XcodeDefault.xctoolchain/usr/lib/"
    ```
