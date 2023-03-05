# Rust-VTF

Rust-VTF is a cross-platform tool for converting between image formats, including support for Valve's VTF format.

### Installation
To install Rust-VTF, you'll need to have Rust installed on your system. Once Rust is installed, you can install Rust-VTF with the following command:

```sh
cargo install rvtf
```

### Usage
#### Command Line
To use Rust-VTF from the command line, run the following command:

```plain
Usage: rvtf [OPTIONS] <INPUT> <OUTPUT>

Arguments:
  <INPUT>   Input file / directory
  <OUTPUT>  Output file / directory

Options:
  -f, --format <FORMAT>  File format (jpg, vtf, etc.)
  -h, --help             Print help
  -V, --version          Print version
```

Here are some examples on how you would use the CLI;

```sh
# Convert a JPEG to a VTF
rvtf input.jpg output.vtf

# Convert a PNG to a JPEG
rvtf input.png output.jpg -f jpg
```

#### Library
To use Rust-VTF as a library, add the following line to your Cargo.toml:

```toml
[dependencies]
rvtf = "1.0.0"
```

Then, in your code, you can use Rust-VTF as follows:

```rust
use rvtf::convert;

// Convert a single file, it will automatically detect the format
convert("input.jpg", "output.vtf", None).unwrap();

// Convert all files in a directory and its subdirectories
convert("input_directory", "output_directory", Some("png")).unwrap();
```


### License

Licensed under either of
 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you shall be dual licensed as above, without any
additional terms or conditions.