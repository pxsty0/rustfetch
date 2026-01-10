##### Note: This code was written almost months ago, but I could only publish it now :D

# rustfetch

A fast, minimal, and colorful system information tool written in Rust, inspired by neofetch. Designed for terminal enthusiasts who want a beautiful and informative summary of their system at a glance.

## Features

- Rust logo with color blocks
- Displays user, host, OS, kernel, uptime, shell, terminal, package count, resolution, DE/WM, CPU, GPU, and memory
- Clean, aligned output for easy reading
- Cross-platform support (Linux, macOS, Windows)
- Minimal dependencies, fast startup

## Installation

### Prerequisites

- Rust toolchain (https://rustup.rs)

### Build from Source

```sh
git clone https://github.com/pxsty0/rustfetch.git
cd rustfetch
cargo build --release
```

The binary will be in `target/release/rustfetch`.

## Usage

```sh
cargo run
# or, after building
./target/release/rustfetch
```

## Example Output

```
                          pxsty@pxstys-MacBook-Air.local (Mac OS)
        @@@@@@@@@@         OS Mac OS 26.1.0
    @ @@@@ @@@  @@@@@@     Kernel 25.1.0
   @@@@            @@@@    Uptime 56 gün, 20 saat, 21 dk
  @@@@@@@@@@@@@@@@  @@@@   Shell zsh
  @  @ @@@@    @@@@ @ @@   Terminal vscode
  @@   @@@@@@@@@@@@  @@@   Packages brew (60)
 @@@   @@@@@@@@@@@    @@@  Resolution 2560x1600
 @@@   @@@@    @@@@  @@@@  DE/WM Aqua
  @@@@@@@@@@@  @@@@@@@@@   CPU Apple M1 (3204 MHz)
   @@@              @@@    GPU Apple M1
    @@@ @        @ @@@     Memory 10.0 GiB / 16.0 GiB
      @@@@@@@@@@@@@@        ■ ■ ■ ■ ■ ■ ■ ■
        @ @ @@@@ @
```

## Contributing

Contributions are welcome! Please open issues or pull requests for bug fixes, features, or improvements.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/fooBar`)
3. Commit your changes (`git commit -am 'Add some fooBar'`)
4. Push to the branch (`git push origin feature/fooBar`)
5. Create a new Pull Request

## License

MIT

---

**Developer:** [pxsty0](https://github.com/pxsty0)
