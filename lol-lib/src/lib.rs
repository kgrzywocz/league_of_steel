#[repr(C)]
pub struct LolStats {
  pub health: u8,
  pub mana: u8,
  pub hit: u8,
}

extern "C" {
  fn lollib_init() -> i32;
  fn lollib_screen_width() -> i32;
  fn lollib_screen_height() -> i32;
  fn lollib_has_mode_changed() -> i32;
  fn lollib_get_stats() -> LolStats;
  fn lollib_is_lol_running() -> i32;
}

pub struct LolLib {}

impl LolLib {
  pub fn init() -> Self {
    unsafe { lollib_init() };

    let width = unsafe { lollib_screen_width() };
    let height = unsafe { lollib_screen_height() };

    log::debug!("Screen {}x{}", width, height);
    Self {}
  }

  pub fn has_mode_changed(&self) -> bool {
    let res = unsafe { lollib_has_mode_changed() };
    res != 0
  }

  pub fn get_stats(&self) -> LolStats {
    let res = unsafe { lollib_get_stats() };
    res
  }

  pub fn is_lol_running() -> bool {
    let res = unsafe { lollib_is_lol_running() };
    res != 0
  }
}
