#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use game_lib::GameTrait;
use games::*;
use games_connector::*;
use hw_connector::wait_for_steel_connector;
use league_of_steel::*;
use league_of_steel_config::*;

fn main() {
    #[cfg(debug_assertions)]
    activate_logger();

    let config = LeagueOfSteelConfig::new();
    let lollib = LolLib::new();
    let games: Vec<&dyn GameTrait> = vec![&lollib];

    let mut game_connector = GamesConnector::new(games);

    let game_infos = game_connector.get_games_info();
    let steel_connector = wait_for_steel_connector(config.sse_seek_interval, &game_infos);
    loop {
        game_connector.check_game_status(&steel_connector, &config);
    }
}

#[cfg(debug_assertions)]
fn activate_logger() {
    if simple_logging::log_to_file("league_of_steel.log", log::LevelFilter::Debug).is_err() {
        eprintln!("Unable to activate logger!");
    }
}
