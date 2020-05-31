use crate::league_of_steel_config::LeagueOfSteelConfig;
use game_lib::*;
use log;

pub struct GamesConnector<'a> {
    games: Vec<&'a dyn GameTrait>,
    active_game: Option<ActiveGame<'a>>,
}

impl<'a> GamesConnector<'a> {
    pub fn new(games: Vec<&'a dyn GameTrait>) -> Self {
        Self {
            games: games,
            active_game: None,
        }
    }
    pub fn get_games_info(&self) -> Vec<game_events::GameInfo> {
        let mut res = vec![];
        for game in &self.games {
            res.push(game.get_game_info())
        }
        res
    }

    pub fn check_game_status(
        &mut self,
        steel_connector: &dyn HwConnector,
        config: &LeagueOfSteelConfig,
    ) {
        if let Some(current_game) = &mut self.active_game {
            log::debug!("active game present");
            if current_game.is_still_running() {
                log::debug!("active game still running");
                current_game.on_game_running(steel_connector);
                std::thread::sleep(config.update_interval);
            } else {
                self.on_game_stop();
                std::thread::sleep(config.game_seek_interval);
            }
        } else {
            let game_running = Self::get_game_running(&self.games);
            log::debug!("game running:{}", game_running.is_some());

            if let Some(game) = game_running {
                let mut active_game = ActiveGame::new(game, game.create_game_analyzer());
                active_game.on_game_running(steel_connector);
                self.active_game = Some(active_game);
                std::thread::sleep(config.update_interval);
            } else {
                std::thread::sleep(config.game_seek_interval);
            }
        }
    }

    fn get_game_running(games: &Vec<&'a dyn GameTrait>) -> Option<&'a dyn GameTrait> {
        for game in games {
            if game.is_running() {
                return Some(*game);
            }
        }
        return None;
    }

    fn on_game_stop(&mut self) {
        self.active_game = None;
    }
}

struct ActiveGame<'a> {
    game: &'a dyn GameTrait,
    analyzer: Box<dyn GameAnalyzer>,
}
impl<'a> ActiveGame<'a> {
    fn new(game: &'a dyn GameTrait, analyzer: Box<dyn GameAnalyzer>) -> Self {
        Self { game, analyzer }
    }

    fn is_still_running(&self) -> bool {
        self.game.is_running()
    }

    fn on_game_running(&mut self, steel_connector: &dyn HwConnector) {
        let events = self.analyzer.pool_events();
        let res = steel_connector.send_events(events);
        if let Err(e) = res {
            log::warn!("{}", e);
        }
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
        let sut = GamesConnector::new(vec![&game_mock_1]);
        assert_eq!(sut.get_games_info().len(), 1);
    }
    #[test]
    fn test_two_game_registration() {
        let game_mock_1 = create_game_mock();
        let game_mock_2 = create_game_mock();
        let sut = GamesConnector::new(vec![&game_mock_1, &game_mock_2]);
        assert_eq!(sut.get_games_info().len(), 2);
    }

    #[test]
    fn test_single_game_start() {
        let game_mock_1 = expect_create_game_analyzer(1, 0);
        let hw_mock = create_hw_mock_with_n_sends(1);
        let config = LeagueOfSteelConfig::new();

        let mut sut = GamesConnector::new(vec![&game_mock_1]);

        sut.check_game_status(&hw_mock, &config);
    }
    #[test]
    fn test_single_when_game_running_no_new_analyzer_created() {
        let mut game_mock_1 = expect_create_game_analyzer(2, 0);
        game_mock_1.expect_is_running().returning(|| true);
        let hw_mock = create_hw_mock_with_n_sends(2);
        let config = LeagueOfSteelConfig::new();

        let mut sut = GamesConnector::new(vec![&game_mock_1]);

        sut.check_game_status(&hw_mock, &config);
        sut.check_game_status(&hw_mock, &config);
    }
    #[test]
    fn test_single_no_pool_after_game_stop() {
        let game_mock_1 = expect_create_game_analyzer(1, 2);
        let hw_mock = create_hw_mock_with_n_sends(1);
        let config = LeagueOfSteelConfig::new();

        let mut sut = GamesConnector::new(vec![&game_mock_1]);

        sut.check_game_status(&hw_mock, &config);
        sut.check_game_status(&hw_mock, &config);
        sut.check_game_status(&hw_mock, &config);
    }
    #[test]
    fn test_second_game_start() {
        let mut game_mock_1 = create_game_mock();
        game_mock_1.expect_is_running().returning(|| false);
        let mut game_mock_2 = expect_create_game_analyzer(2, 0);
        game_mock_2.expect_is_running().returning(|| true);
        let hw_mock = create_hw_mock_with_n_sends(2);
        let config = LeagueOfSteelConfig::new();

        let mut sut = GamesConnector::new(vec![&game_mock_1, &game_mock_2]);

        sut.check_game_status(&hw_mock, &config);
        sut.check_game_status(&hw_mock, &config);
    }

    #[test]
    fn test_game_changed() {
        let mut seq = Sequence::new();

        let hw_mock = create_hw_mock_with_n_sends(2);
        let config = LeagueOfSteelConfig::new();
        let mut game_mock_1 = create_game_mock();
        let mut game_mock_2 = create_game_mock();
        game_mock_1
            .expect_is_running()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| false);
        game_mock_2
            .expect_is_running()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| true);
        game_mock_2
            .expect_create_game_analyzer()
            .returning(move || Box::new(create_analyzer_mock_with_n_status_pools(1)));

        game_mock_2
            .expect_is_running()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| false);
        game_mock_1
            .expect_is_running()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| true);
        game_mock_1
            .expect_create_game_analyzer()
            .returning(move || Box::new(create_analyzer_mock_with_n_status_pools(1)));

        let mut sut = GamesConnector::new(vec![&game_mock_1, &game_mock_2]);
        sut.check_game_status(&hw_mock, &config);

        sut.check_game_status(&hw_mock, &config);
        sut.check_game_status(&hw_mock, &config);
    }

    fn expect_create_game_analyzer(n_status_pools: usize, n_stop_pools: usize) -> MockGame {
        let mut seq = Sequence::new();
        let mut game_mock_1 = create_game_mock();
        game_mock_1
            .expect_is_running()
            .times(1)
            .in_sequence(&mut seq)
            .returning(|| true);
        game_mock_1
            .expect_create_game_analyzer()
            .times(1)
            .in_sequence(&mut seq)
            .returning(move || Box::new(create_analyzer_mock_with_n_status_pools(n_status_pools)));
        game_mock_1
            .expect_is_running()
            .returning(|| false)
            .times(n_stop_pools)
            .in_sequence(&mut seq);
        game_mock_1
    }
    fn create_game_mock() -> MockGame {
        let mut game_mock = MockGame::new();
        game_mock
            .expect_get_game_info()
            .times(0..2)
            .returning(|| GameInfo::new("GAME1", "game1", "dev", vec![]));
        game_mock
    }
    fn create_analyzer_mock_with_n_status_pools(number_of_pools: usize) -> MockAnalyzer {
        let mut mock_analyzer = MockAnalyzer::new();
        mock_analyzer
            .expect_pool_events()
            .times(number_of_pools)
            .returning(|| MultipleGameEvents::new("GAME1", vec![]));
        mock_analyzer.expect_drop().returning(|| ());
        mock_analyzer
    }
    fn create_hw_mock_with_n_sends(number_of_sends: usize) -> MockHw {
        let mut hw_mock = MockHw::new();
        hw_mock
            .expect_send_events()
            .times(number_of_sends)
            .returning(|events| {
                assert_eq!(events.game, "GAME1");
                Ok(())
            });
        hw_mock
    }

    mock! {
        Game{}
        trait GameTrait {
            fn get_game_info(&self) -> GameInfo;
            fn is_running(&self) -> bool;
            fn create_game_analyzer(&self) -> Box<dyn GameAnalyzer>;
        }
    }
    mock! {
        Analyzer{}
        trait GameAnalyzer {
            fn pool_events(&mut self) -> MultipleGameEvents;
        }
        trait Drop{
            fn drop(&mut self);
        }
    }
    mock! {
        Hw{}
        trait HwConnector{
            fn register_games(&self, game_infos: &Vec<GameInfo>) -> Result<(), HwError>;
            fn register(&self, game_info: &GameInfo) -> Result<(), HwError>;
            fn send_events(&self, events: MultipleGameEvents) -> Result<(), HwError>;
        }
    }
}
