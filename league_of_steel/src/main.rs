#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use league_of_steel::*;
use std::time::Duration;

const SSE_SEEK_INTERVAL: Duration = Duration::from_millis(1000);
const GAME_SEEK_INTERVAL: Duration = Duration::from_millis(1000);
const UPDATE_INTERVAL: Duration = Duration::from_millis(150);

fn main() {
    #[cfg(debug_assertions)]
    activate_logger();

    let steel_connector = wait_for_steel_connector(SSE_SEEK_INTERVAL);
    let mut lol_lib = None;

    loop {
        let is_lol_running = lol_lib::LolLib::is_lol_running();
        log::debug!("lol running:{}", is_lol_running);

        if is_lol_running {
            on_game_running(&mut lol_lib, &steel_connector);
            std::thread::sleep(UPDATE_INTERVAL);
        } else {
            if lol_lib.is_some() {
                on_game_stop(&mut lol_lib);
            }
            std::thread::sleep(GAME_SEEK_INTERVAL);
        }
    }
}

#[cfg(debug_assertions)]
fn activate_logger() {
    if simple_logger::init_with_level(log::Level::Debug).is_err() {
        eprintln!("Unable to activate logger!");
    }
}
