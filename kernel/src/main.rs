#![no_std]
#![no_main]

bootloader_api::entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
  if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
    for byte in framebuffer.buffer_mut() {
      *byte = 0x90;
    }
  }
  loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}