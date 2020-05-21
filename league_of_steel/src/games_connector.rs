use game_lib::*;
use log;
use std::borrow::BorrowMut;
use std::ops::DerefMut;
use steel_lib;

pub struct GamesConnector {
    games: Vec<Box<dyn GameTrait>>,
    game_analyzer_opt: Option<Box<dyn GameAnalyzer>>,
}

impl GamesConnector {
    pub fn new(games: Vec<Box<dyn GameTrait>>) -> Self {
        Self {
            games: games,
            game_analyzer_opt: None,
        }
    }
    pub fn get_games_info(&self) -> Vec<game_events::GameInfo> {
        let mut res = vec![];
        for game in &self.games {
            res.push(game.get_game_info())
        }
        res
    }

    pub fn is_game_running(&self) -> bool {
        self.games[0].is_running()
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
        self.optionally_re_init_game_analyzer();
        if let Some(game) = &mut self.game_analyzer_opt.borrow_mut() {
            return fun(game.deref_mut());
        };
        panic!("Unable to init game lib!")
    }
    fn optionally_re_init_game_analyzer(&mut self) {
        match &self.game_analyzer_opt {
            None => self.re_init_game_analyzer(),
            Some(_) => {}
        };
    }
    fn re_init_game_analyzer(&mut self) {
        self.game_analyzer_opt = None;
        self.game_analyzer_opt = Some(self.games[0].create_game_analyzer());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use game_lib::game_events::*;
    use mockall::*;

    #[test]
    fn test_zero_games_registration() {
        let sut = GamesConnector::new(vec![]);
        assert_eq!(sut.get_games_info().len(), 0);
    }
    #[test]
    fn test_single_game_registration() {
        let game_mock_1 = create_game_mock();
        let sut = GamesConnector::new(vec![Box::new(game_mock_1)]);
        assert_eq!(sut.get_games_info().len(), 1);
    }
    #[test]
    fn test_two_game_registration() {
        let game_mock_1 = create_game_mock();
        let game_mock_2 = create_game_mock();
        let sut = GamesConnector::new(vec![Box::new(game_mock_1), Box::new(game_mock_2)]);
        assert_eq!(sut.get_games_info().len(), 2);
    }

    fn create_game_mock() -> MockGame {
        let mut game_mock = MockGame::new();
        game_mock
            .expect_get_game_info()
            .returning(|| GameInfo::new("g1", "game1", "dev", vec![]));
        game_mock
    }

    mock! {
        Game{}
        trait GameTrait {
            fn get_game_info(&self) -> GameInfo;
            fn is_running(&self) -> bool;
            fn create_game_analyzer(&self) -> Box<dyn GameAnalyzer>;
        }
    }
}
