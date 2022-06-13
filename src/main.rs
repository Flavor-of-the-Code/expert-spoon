mod player;

fn main() {
  let mut noob_dennis: player::Player = player::Player::new("noob_dennis");
  assert_eq!(noob_dennis.total_xp_needed(), 1000);
  noob_dennis.apply_xp(10001);
  assert_eq!(noob_dennis.lvl(), 4);
  assert_eq!(noob_dennis.xp(), 977);
  assert_eq!(noob_dennis.total_xp_needed(), 8000);
}
