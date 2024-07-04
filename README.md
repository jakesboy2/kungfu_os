# KungfuOS
Blazingly fast operating system built in Rust

----

### Setup Instructions

1. Ensure you have Rust and relevant toolchains installed (rustup, cargo, neovim)
2. Install qemu
    a. MacOS - `brew install qemu`
    b. Windows - [See instructions here](https://www.qemu.org/download/#windows)
3. `git clone https://github.com/jakesboy2/kungfu_os.git && cd kungfu_os`
4. `cargo build`, not strictly needed as running will build, but nice to check for errors
5. Run the application

### Ways to Run
* `cargo run` - Generate .img files for uefi and bios, print file path to stdout
* `cargo run --bin qemu-bios` - Generate .img files for uefi and bios, launch the bios file in qemu
* `cargo run --bin qemu-uefi` - Generate .img files for uefi and bios, launch the uefi file in qemu
    * The uefi file requires an additional file passed using the --bios flag. This is included via a rust crate and requires no action on the user's part, but is important to note if you are invoking the .img file directly from the CLI.

