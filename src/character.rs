// Items in a module default to private visibility. Override with the `pub` modifier.
// **Structs:** have an extra level of visibility with their fields. Visibility defaults to private, can be overridden with the `pub` modifier. The visibility only matters when accessed from outside the module and has the goal of hiding information.

use std::ops::Mul;

pub struct SCharacter {
  name: &'static str,
  lvl: u32,
  xp: u32,
}

pub trait TCharacter {
  fn new(name: &'static str) -> Self;

  /**
   * Method signatures.
   * Need to actually be **implemented**.
   */

  fn name(&self) -> &'static str;
  fn lvl(&self) -> u32;
  fn xp(&self) -> u32;
  fn add_xp(&mut self, amount: u32) -> bool;

  /**
   * Traits can provide default method definitions.
   * Do **not** need to be implemented, but can be overridden.
   */

  fn can_lvl_up(&self) -> bool {
    self.xp() >= self.total_xp_needed()
  }

  fn total_xp_needed(&self) -> u32 {
    let exponent = 1.5;
    let base_xp = 1000.0;
    let level_pow_exponent = (self.lvl() as f32).powf(exponent);
    base_xp.mul(level_pow_exponent) as u32
  }
}

impl TCharacter for SCharacter {
  fn new(name: &'static str) -> SCharacter {
    SCharacter {
      name,
      lvl: 1,
      xp: 0,
    }
  }

  fn name(&self) -> &'static str {
    self.name
  }

  fn lvl(&self) -> u32 {
    self.lvl
  }

  fn xp(&self) -> u32 {
    self.xp
  }

  fn add_xp(&mut self, amount: u32) -> bool {
    self.xp += amount;
    self.can_lvl_up()
  }
}
