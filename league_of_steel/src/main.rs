#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use game_lib::GameTrait;
use games::*;
use games_connector::*;
use hw_connector::wait_for_steel_connector;
use league_of_steel::*;
use std::time::Duration;

const SSE_SEEK_INTERVAL: Duration = Duration::from_millis(1000);
const GAME_SEEK_INTERVAL: Duration = Duration::from_millis(1000);
const UPDATE_INTERVAL: Duration = Duration::from_millis(150);

fn main() {
    #[cfg(debug_assertions)]
    activate_logger();

    let games: Vec<Box<dyn GameTrait>> = vec![Box::new(LolLib::new())];

    let mut game_connector = GamesConnector::new(games);
    let game_infos = game_connector.get_games_info();
    let steel_connector = wait_for_steel_connector(SSE_SEEK_INTERVAL, &game_infos);

    loop {
        let is_game_running = game_connector.is_game_running();
        log::debug!("game running:{}", is_game_running);

        if is_game_running {
            game_connector.on_game_running(&steel_connector);
            std::thread::sleep(UPDATE_INTERVAL);
        } else {
            game_connector.on_game_stop();
            std::thread::sleep(GAME_SEEK_INTERVAL);
        }
    }
}

#[cfg(debug_assertions)]
fn activate_logger() {
    if simple_logging::log_to_file("league_of_steel.log", log::LevelFilter::Debug).is_err() {
        eprintln!("Unable to activate logger!");
    }
}
