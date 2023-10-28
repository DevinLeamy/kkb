# _KKB_

> ### It's just a fact: _**<ins>K</ins>ool <ins>K</ins>ids have nice <ins>B</ins>ackgrounds**_.

<p align="center">
  <img src="https://github.com/DevinLeamy/kkb/assets/45083086/28f492a6-0ffe-4dcf-8c94-b80bf7abd2a9" width="300" height="300">
</p>

<p>
  <a href="#install">Install</a> •
  <a href="#usage">Usage</a> •
  <a href="#options">Options</a> •
  <a href="#tech">Tech</a> •
  <a href="#faq">FAQ</a>
</p>

### Install

```bash
cargo install kkb
```

Set `OPENAI_API_KEY` to your OpenAI API key.
```bash
export OPENAI_API_KEY=<key>
```

Ensure you have OpenCV installed. It can be finiky - _I wish it wasn't_. Installation instructions for each platform can be found [here](https://github.com/twistedfall/opencv-rust).

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

- `-o <path>`: Output image path.

### Tech

-   Image generation: [OpenAI's DALL-E 3](https://openai.com/dall-e-3)
-   Image super resolution: [OpenCV](https://github.com/twistedfall/opencv-rust)

### FAQ

- _"It's taking a while, is something wrong?_". No, great craftsmanship just takes time.
-   [Macos] Can't install OpenCV? Try `brew install opencv`.

    1.  You may need to [reinstall homebrew](https://github.com/Genymobile/scrcpy/issues/2128#issuecomment-958046872) if you recently updated your OS.
    2.  You may need to set this library path:

    ```bash
    export DYLD_FALLBACK_LIBRARY_PATH="$(xcode-select --print-path)/Toolchains/XcodeDefault.xctoolchain/usr/lib/"
    ```
