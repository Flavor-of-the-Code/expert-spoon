use std::ops::Mul;

fn needed_xp(level: u32) -> u32 {
  let exponent = 1.5_f32;
  let base_xp = 1000_f32;
  let level_pow_exponent = (level as f32).powf(exponent);
  base_xp.mul(level_pow_exponent) as u32
}

fn main() {
  assert_eq!(needed_xp(1), 1000);
  assert_eq!(needed_xp(2), 2828);
  assert_eq!(needed_xp(3), 5196);

  println!("Hello, world!");
}
