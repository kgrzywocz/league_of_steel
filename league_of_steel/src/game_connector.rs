use log;
use lol_lib;
use std::borrow::BorrowMut;
use steel_lib;

pub struct GameConnector {
    game_lib_opt: Option<lol_lib::LolLib>,
}

impl GameConnector {
    pub fn new() -> Self {
        Self { game_lib_opt: None }
    }
    pub fn is_game_running(&self) -> bool {
        lol_lib::LolLib::is_running()
    }

    pub fn on_game_running(&mut self, steel_connector: &steel_lib::SteelConnector) {
        let events = self.do_with_game_lib(|lib: &mut lol_lib::LolLib| lib.pool_events());
        let res = steel_connector.send_events(events);
        if let Err(e) = res {
            log::warn!("{}", e);
        }
    }

    pub fn on_game_stop(&mut self) {
        self.game_lib_opt = None;
    }

    fn do_with_game_lib<F, T>(&mut self, fun: F) -> T
    where
        F: Fn(&mut lol_lib::LolLib) -> T,
    {
        self.optionally_re_init_game_lib();
        if let Some(game_lib) = &mut self.game_lib_opt.borrow_mut() {
            return fun(game_lib);
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
