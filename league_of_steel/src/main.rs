#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log;
use lol_lib;
use std::time::Duration;
use steel_lib;
use steel_lib::SteelLibError;

const SSE_SEEK_INTERVAL: Duration = Duration::from_millis(1000);
const GAME_SEEK_INTERVAL: Duration = Duration::from_millis(1000);
const UPDATE_INTERVAL: Duration = Duration::from_millis(150);

fn main() {
    #[cfg(debug_assertions)]
    activate_logger();

    let steel_connector = wait_for_steel_connector();
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

fn wait_for_steel_connector() -> steel_lib::SteelConnector {
    let mut steel_connector;
    loop {
        let res = steel_lib::SteelConnector::new();
        match res {
            Ok(res) => {
                steel_connector = res;
                if let Err(e) = register_game(&steel_connector) {
                    log::warn!("{}", e);
                } else {
                    break;
                };
            }
            Err(e) => {
                log::warn!("{}", e);
            }
        }
        std::thread::sleep(SSE_SEEK_INTERVAL);
    }
    steel_connector
}

fn register_game(steel_connector: &steel_lib::SteelConnector) -> Result<(), SteelLibError> {
    steel_connector.register_game()?;
    steel_connector.register_game_events()
}

fn on_game_running(
    lol_lib_opt: &mut Option<lol_lib::LolLib>,
    steel_connector: &steel_lib::SteelConnector,
) {
    let lol_lib = get_lol_lib(lol_lib_opt);
    let res = lol_stats_update(lol_lib, &steel_connector);
    if let Err(e) = res {
        log::warn!("{}", e);
    }
}

fn get_lol_lib(lol_lib_opt: &mut Option<lol_lib::LolLib>) -> &lol_lib::LolLib {
    optionally_re_init_lol_lib(lol_lib_opt);
    if let Some(lol_lib) = lol_lib_opt {
        return lol_lib;
    };
    panic!("Unable to init lol lib!")
}
fn optionally_re_init_lol_lib(lol_lib_opt: &mut Option<lol_lib::LolLib>) {
    match lol_lib_opt {
        None => *lol_lib_opt = Some(lol_lib::LolLib::init()),
        Some(lol_lib) => {
            let mode_has_changed = lol_lib.has_mode_changed();
            if mode_has_changed {
                log::info!("Mode changed reinitialize lol lib");
                *lol_lib_opt = Some(lol_lib::LolLib::init());
            }
        }
    };
}

fn lol_stats_update(
    lol_lib: &lol_lib::LolLib,
    steel_connector: &steel_lib::SteelConnector,
) -> Result<(), SteelLibError> {
    let stats = lol_lib.get_stats();
    steel_connector.send_health(stats.health)?;
    steel_connector.send_mana(stats.mana)?;
    steel_connector.send_hit(stats.hit)
}

fn on_game_stop(lol_lib_opt: &mut Option<lol_lib::LolLib>) {
    if let Some(lol_lib) = lol_lib_opt {
        lol_lib.destroy();
        *lol_lib_opt = None;
    }
}
