use bootloader::DiskImageBuilder;
use std::{env, path::PathBuf};

fn main() {
  // set by cargo for kernal depedency artifact
  let kernel_path = env::var("CARGO_BIN_FILE_KERNEL").unwrap();
  let disk_builder = DiskImageBuilder::new(PathBuf::from(kernel_path));

  // specify output paths
  let output_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
  let uefi_path = output_dir.join("kungfu_os-uefi.img");
  let bios_path = output_dir.join("kungfu_os-bios.img");

  // create disk images
  disk_builder.create_uefi_image(&uefi_path).unwrap();
  disk_builder.create_bios_image(&bios_path).unwrap();
  
  // pass disk image paths via env variables
  println!("cargo:rustc-env=UEFI_IMAGE={}", uefi_path.display());
  println!("cargo:rustc-env=BIOS_IMAGE={}", bios_path.display());
}