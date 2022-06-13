mod character;
use crate::character::TCharacter;

fn main() {
  let mut dennis: character::SCharacter = character::TCharacter::new("dennis");
  assert_eq!(dennis.total_xp_needed(), 1000);
  assert_eq!(dennis.xp(), 0);
  assert_eq!(dennis.add_xp(500) as i32, 0);
  assert_eq!(dennis.xp(), 500);
}
