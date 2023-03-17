
#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;
static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
  println!("Hello World{}", "!");
  panic!("W panic");

//   loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
  println!("{}", _info);
  loop {}
}
