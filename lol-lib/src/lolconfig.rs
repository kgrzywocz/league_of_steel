pub fn get_hud_global_scale(exe_path: &str) -> Option<f32> {
    let exe_path = std::path::Path::new(exe_path);
    let lol_root = exe_path.parent()?.parent()?;
    let config_path = lol_root.join("Config").join("game.cfg");

    log::debug!("LoL config path = {:?}", config_path);
    let config = std::fs::read(config_path).ok()?;
    let config = String::from_utf8_lossy(&config);

    log::debug!("LoL config: {}", config);

    let re = regex::Regex::new(r#"GlobalScale=(\d+.\d+)"#).expect("Error in regex");
    let scale = re.captures(&config)?.get(1)?.as_str();
    scale.parse().ok()
}
