use log;
use lol_lib;
use steel_lib;
use steel_lib::SteelLibError;

pub fn wait_for_steel_connector(
    sse_seek_interval: std::time::Duration,
) -> steel_lib::SteelConnector {
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
        std::thread::sleep(sse_seek_interval);
    }
    steel_connector
}

fn register_game(steel_connector: &steel_lib::SteelConnector) -> Result<(), SteelLibError> {
    steel_connector.register_game()?;
    steel_connector.register_game_events()
}

pub fn on_game_running(
    lol_lib_opt: &mut Option<lol_lib::LolLib>,
    steel_connector: &steel_lib::SteelConnector,
) {
    let lol_lib = get_lol_lib(lol_lib_opt);
    lol_set_hud_scale(lol_lib);
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

fn lol_set_hud_scale(lol_lib: &lol_lib::LolLib) {
    let hud_scale = lol_lib.get_hud_global_scale_from_config().unwrap_or(1.0);
    log::debug!("hud_scale={}", hud_scale);
    lol_lib.set_hud_scaling(hud_scale);
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

pub fn on_game_stop(lol_lib_opt: &mut Option<lol_lib::LolLib>) {
    if let Some(lol_lib) = lol_lib_opt {
        lol_lib.destroy();
        *lol_lib_opt = None;
    }
}
