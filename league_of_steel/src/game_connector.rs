use log;
use lol_lib;
use steel_lib;
use steel_lib::SteelLibError;

pub struct GameConnector {
    game_lib_opt: Option<lol_lib::LolLib>,
}

impl GameConnector {
    pub fn new() -> Self {
        Self { game_lib_opt: None }
    }
    pub fn is_game_running(&self) -> bool {
        lol_lib::LolLib::is_lol_running()
    }

    pub fn on_game_running(&mut self, steel_connector: &steel_lib::SteelConnector) {
        let stats = self.get_game_lib().get_stats();
        let res = steel_connector.send_stats(stats.health, stats.mana, stats.hit);
        if let Err(e) = res {
            log::warn!("{}", e);
        }
    }

    pub fn on_game_stop(&mut self) {
        self.game_lib_opt = None;
    }

    fn get_game_lib(&mut self) -> &lol_lib::LolLib {
        self.optionally_re_init_game_lib();
        if let Some(game_lib) = &self.game_lib_opt {
            return &game_lib;
        };
        panic!("Unable to init game lib!")
    }
    fn optionally_re_init_game_lib(&mut self) {
        match &self.game_lib_opt {
            None => self.re_init_game_lib(),
            Some(game_lib) => {
                let mode_has_changed = game_lib.has_mode_changed();
                if mode_has_changed {
                    log::info!("Mode changed reinitialize game lib");
                    self.re_init_game_lib();
                }
            }
        };
    }
    fn re_init_game_lib(&mut self) {
        self.game_lib_opt = None;
        self.game_lib_opt = Some(lol_lib::LolLib::new());
    }
}
