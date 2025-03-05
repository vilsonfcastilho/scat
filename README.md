# scat

A simple and efficient Command Line Application written in Rust that converts a `.jpg` image into an ASCII art representation directly in your terminal. 🎨🦀

## Features ✨

- 🔄️ Convert `.jpg` images into ASCII format
- ↗️ Customizable output width with the `--width` parameter
- 🚀 Fast and lightweight, thanks to Rust's performance
- 📟 Easy to use via command line

## Output 🎨

```
@@@@@@@@@@@%%%%%%%##****++=--::..
@@@@@@@@@%%%##****++==--::...
@@@@@@%%%##***++==--::.....
@@@@%%##***++==--::.......
@@@##***++==--:::.......
@#**++==--:::........
*++==--:::...........
==--::...............
-::.................
```

## Usage 🕹️

```bash
$ scat <path_to_jpg_image> --width <number>
```

### Example

```bash
$ scat ./assets/example/image.jpg --width 100
```

This will output the ASCII art of `image.jpg` with a width of `100` characters directly into your terminal.

## Prerequisites 📋

To build and run the project, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/) (latest stable version recommended)
- Cargo (Rust's package manager, included with Rust)

## Getting Started 🚀

### 1. Clone the Repository

```bash
$ git clone https://github.com/vilsonfcastilho/scat.git
$ cd scat
```

### 2. Build the project

```bash
$ cargo build --release
```

### 3. Run the application

By default the width output is `80`.

```bash
$ cargo run path/to/image.jpg
```

Run the project specifying a different width output.

```bash
$ cargo run path/to/image.jpg --width 100
```

Or use the binary after build.

```bash
$ ./target/release/scat path/to/image.jpg --width 100
```

## Dependencies 📦

This project uses the following Rust crates:

- [`clap`](https://crates.io/crates/clap) - A simple to use, efficient, and full-featured Command Line Argument Parser.
- [`image`](https://crates.io/crates/image) - Imaging library. Provides basic image processing and encoders/decoders for common image formats.

Add these dependencies to your `Cargo.toml` file if you're setting up manually:

```toml
[dependencies]
clap = { version = "4.5.27", features = ["derive"] }
image = "0.25.5"
```

## Contact 📧

For any inquiries or feedback, feel free to reach out via GitHub or email.

---

Made with ♥ by Vilson Castilho
