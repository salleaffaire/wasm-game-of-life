fn perform_xor(x: i32, y: i32) -> i32 {
  x ^ y
}

use std::process;

fn main() -> ! {
  process::exit(perform_xor(42, 24))
}
