extern crate libc;
use backend;

mod lolconfig;

#[repr(C)]
pub struct LolStats {
    pub health: u8,
    pub mana: u8,
    pub hit: u8,
}

extern "C" {
    fn lollib_init() -> i32;
    fn lollib_destroy();
    fn lollib_screen_width() -> i32;
    fn lollib_screen_height() -> i32;
    fn lollib_has_mode_changed() -> i32;
    fn lollib_get_stats() -> LolStats;
    fn lollib_set_hud_scaling(hud_global_scale: f32);
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
    pub fn destroy(&self) {
        unsafe { lollib_destroy() };
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
        backend::is_lol_running()
    }

    pub fn set_hud_scaling(&self, hud_global_scale: f32) {
        unsafe { lollib_set_hud_scaling(hud_global_scale) };
    }

    pub fn get_hud_global_scale_from_config(&self) -> Option<f32> {
        lolconfig::get_hud_global_scale(&backend::lol_exe_path())
    }
}
