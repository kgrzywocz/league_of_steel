use game_lib::*;
use games;
use log;
use std::borrow::BorrowMut;
use std::ops::DerefMut;
use steel_lib;

pub struct GameConnector {
    game_lib: Box<dyn GameTrait>,
    game_analyzer_opt: Option<Box<dyn GameAnalyzer>>,
}

impl GameConnector {
    pub fn new() -> Self {
        Self {
            game_lib: Box::new(games::LolLib::new()),
            game_analyzer_opt: None,
        }
    }
    pub fn get_games(&self) -> Vec<game_events::GameInfo> {
        vec![self.game_lib.get_game_info()]
    }

    pub fn is_game_running(&self) -> bool {
        self.game_lib.is_running()
    }

    pub fn on_game_running(&mut self, steel_connector: &steel_lib::SteelConnector) {
        let events = self.do_with_game(|analyzer: &mut dyn GameAnalyzer| analyzer.pool_events());
        let res = steel_connector.send_events(events);
        if let Err(e) = res {
            log::warn!("{}", e);
        }
    }

    pub fn on_game_stop(&mut self) {
        self.game_analyzer_opt = None;
    }

    fn do_with_game<F, T>(&mut self, fun: F) -> T
    where
        F: Fn(&mut dyn GameAnalyzer) -> T,
    {
        self.optionally_re_init_game_lib();
        if let Some(game) = &mut self.game_analyzer_opt.borrow_mut() {
            return fun(game.deref_mut());
        };
        panic!("Unable to init game lib!")
    }
    fn optionally_re_init_game_lib(&mut self) {
        match &self.game_analyzer_opt {
            None => self.re_init_game_lib(),
            Some(_) => {}
        };
    }
    fn re_init_game_lib(&mut self) {
        self.game_analyzer_opt = None;
        self.game_analyzer_opt = Some(self.game_lib.create_game_analyzer());
    }
}
