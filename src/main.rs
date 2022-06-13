mod character;
use crate::character::TCharacter;

fn main() {
  let mut dennis: character::SCharacter = character::TCharacter::new("dennis");
  assert_eq!(dennis.total_xp_needed(), 1000);
  dennis.apply_xp(10001);
  assert_eq!(dennis.lvl(), 4);
}
