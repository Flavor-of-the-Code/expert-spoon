mod character;
use crate::character::TCharacter;

mod noob;

fn main() {
  // let mut dennis: character::SCharacter = character::TCharacter::new("dennis");
  // assert_eq!(dennis.total_xp_needed(), 1000);
  // dennis.apply_xp(10001);
  // assert_eq!(dennis.lvl(), 4);
  // assert_eq!(dennis.xp(), 977);
  // assert_eq!(dennis.total_xp_needed(), 8000);

  let mut noob_dennis: noob::Noob = noob::Noob::new("noob_dennis");
  assert_eq!(noob_dennis.total_xp_needed(), 1000);
  noob_dennis.apply_xp(10001);
  assert_eq!(noob_dennis.lvl(), 4);
  assert_eq!(noob_dennis.xp(), 977);
  assert_eq!(noob_dennis.total_xp_needed(), 8000);
}
