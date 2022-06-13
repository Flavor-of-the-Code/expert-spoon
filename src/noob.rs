use std::ops::Mul;

pub struct Noob {
  name: &'static str,
  lvl: u32,
  xp: i32,
}

impl Noob {
  pub fn new(name: &'static str) -> Noob {
    Noob {
      name,
      lvl: 1,
      xp: 0,
    }
  }

  pub fn name(&self) -> &'static str {
    self.name
  }

  pub fn lvl(&self) -> u32 {
    self.lvl
  }

  pub fn xp(&self) -> i32 {
    self.xp
  }

  fn add_xp(&mut self, amount: i32) {
    self.xp += amount;
  }

  fn apply_lvl(&mut self, gain: (i32, u32)) {
    let (xp, lvl) = gain;
    self.xp += xp;
    self.lvl += lvl;
  }

  pub fn apply_xp(&mut self, amount: i32) {
    self.add_xp(amount);
    while self.can_lvl_up() {
      let gain = self.create_lvl_up();
      self.apply_lvl(gain);
      self.print();
    }
  }

  fn can_lvl_up(&self) -> bool {
    self.xp() >= self.total_xp_needed()
  }

  pub fn total_xp_needed(&self) -> i32 {
    let exponent = 1.5;
    let base_xp = 1000.0;
    let level_pow_exponent = (self.lvl() as f32).powf(exponent);
    base_xp.mul(level_pow_exponent) as i32
  }

  fn create_lvl_up(&self) -> (i32, u32) {
    let xp = -self.total_xp_needed();
    let lvl = 1;
    (xp, lvl)
  }

  fn print(&self) {
    println!("------------------------------");
    println!("Name: {:?}", self.name());
    println!("Level: {:?}", self.lvl());
    println!(
      "XP: {:?} / {:?} ({:?})",
      self.xp(),
      self.total_xp_needed(),
      self.total_xp_needed() - self.xp()
    );
    println!("------------------------------");
  }
}
