use testlib::*;

#[ignore]
#[test]
fn test_1280x720() {
    test_resolution(1280, 720, "screens/1280x720_zed_hp100_energy100.jpg");
}
#[ignore]
#[test]
fn test_1920x1080() {
    test_resolution(1920, 1080, "screens/1920x1080_hp100_mana100.png");
}
#[ignore]
#[test]
fn test_1280x1024() {
    test_resolution(1280, 1024, "screens/1280x1024_hp100_mana100.png");
}
#[ignore]
#[test]
fn test_800x600() {
    test_resolution(800, 600, "screens/800x600_hp100_mana100.png");
}
#[ignore]
#[test]
fn test_1024x768() {
    test_resolution(1024, 768, "screens/1024x768_hp100_mana100.png");
}
#[ignore]
#[test]
fn test_1152x864() {
    test_resolution(1152, 864, "screens/1152x864_hp100_mana100.png");
}

#[ignore]
#[test]
fn test_1920x1080_hud_50() {
    let _config = LolConfig::create_with_hud_scale("0.5000");
    test_resolution(1920, 1080, "screens/1920x1080_hud_globalScale_0_5000.png");
}
#[ignore]
#[test]
fn test_1920x1080_hud_0() {
    let _config = LolConfig::create_with_hud_scale("0.0100");
    test_resolution(1920, 1080, "screens/1920x1080_hud_globalScale_0_0100.png");
}
#[ignore]
#[test]
fn test_1024x768_hud_0() {
    let _config = LolConfig::create_with_hud_scale("0.0100");
    test_resolution(1024, 768, "screens/1024x768_hud_globalScale_0_0100.png");
}
#[ignore]
#[test]
fn test_1600x1024_hud_0() {
    let _config = LolConfig::create_with_hud_scale("0.0100");
    test_resolution(1600, 1024, "screens/1600x1024_hud_globalScale_0_0100.png");
}
#[ignore]
#[test]
fn test_1680x1050_hud_0() {
    let _config = LolConfig::create_with_hud_scale("0.0100");
    test_resolution(1680, 1050, "screens/1680x1050_hud_globalScale_0_0100.png");
}
#[ignore]
#[test]
fn test_800x600_hud_0() {
    let _config = LolConfig::create_with_hud_scale("0.0100");
    test_resolution(800, 600, "screens/800x600_hud0_hp100_mana100.png");
}

fn test_resolution(width: u32, height: u32, file: &'static str) {
    let window = image::ImageDisplay::new(width, height, file);
    let gamesensestub = server::ServerStub::new();

    let _sut = start_sut();

    let _lol = start_lol();
    expect_game_register(&gamesensestub);
    expect_game_events(&gamesensestub, [100, 100, 0]);

    window.stop();
}

#[ignore]
#[test]
fn test_critical_hit() {
    let _window = show_critical_hit_image();
    let _lol = start_lol();
    let gamesensestub = server::ServerStub::new();

    let _sut = start_sut();

    expect_game_register(&gamesensestub);
    expect_game_events(&gamesensestub, [49, 100, 19]);
}

#[ignore]
#[test]
fn should_work_after_game_restart() {
    let _window = show_critical_hit_image();
    let mut lol = start_lol();
    let gamesensestub = server::ServerStub::new();

    let _sut = start_sut();

    expect_game_register(&gamesensestub);
    expect_game_events(&gamesensestub, [49, 100, 19]);

    lol.stop();
    let _window = image::ImageWindow::new(1920, 1080, "screens/1920x1080_hp100_mana100.png");
    lol = start_lol();
    expect_game_events(&gamesensestub, [100, 100, 0]);
    lol.stop();
}

#[ignore]
#[test]
fn should_work_after_resolution_change() {
    let _window = show_critical_hit_image();
    let _lol = start_lol();
    let gamesensestub = server::ServerStub::new();

    let _sut = start_sut();

    expect_game_register(&gamesensestub);
    expect_game_events(&gamesensestub, [49, 100, 19]);

    let _window = image::ImageWindow::new(1280, 1024, "screens/1280x1024_hp100_mana100.png");
    ignore_game_events(&gamesensestub);

    expect_game_events(&gamesensestub, [100, 100, 0]);
}

#[ignore]
#[test]
fn should_wait_for_sse3() {
    let _sut = start_sut();
    let _window = show_critical_hit_image();
    let _lol = start_lol();
    let gamesensestub = server::ServerStub::new();

    expect_game_register(&gamesensestub);
    expect_game_events(&gamesensestub, [49, 100, 19]);
}

fn start_sut() -> Process {
    Process::start("../target/debug/league_of_steel.exe")
}

fn show_critical_hit_image() -> image::ImageWindow {
    image::ImageWindow::new(1920, 1080, "screens/1920x1080_critical_hit.png")
}

fn expect_game_register(gamesensestub: &server::ServerStub) {
    expect_lol_register(gamesensestub);
    expect_fortnite_register(gamesensestub);
}

fn expect_lol_register(gamesensestub: &server::ServerStub) {
    gamesensestub.expect_request("/game_metadata", "League of Legends");
    gamesensestub.expect_request("/register_game_event", "HEALTH");
    gamesensestub.expect_request("/register_game_event", "MANA");
    gamesensestub.expect_request("/register_game_event", "HIT");
    gamesensestub.expect_request("/register_game_event", "Q_SPELL");
    gamesensestub.expect_request("/register_game_event", "W_SPELL");
    gamesensestub.expect_request("/register_game_event", "E_SPELL");
    gamesensestub.expect_request("/register_game_event", "R_SPELL");
}

fn expect_fortnite_register(gamesensestub: &server::ServerStub) {
    gamesensestub.expect_request("/game_metadata", "Fortnite");
    gamesensestub.expect_request("/register_game_event", "HEALTH");
    gamesensestub.expect_request("/register_game_event", "ARMOR");
}

fn expect_game_events(gamesensestub: &server::ServerStub, stats: [u8; 3]) {
    gamesensestub.expect_request(
        "/multiple_game_events",
        &format!(
            "HEALTH.*value.* {}.*MANA.*value.* {}.*HIT.*value.* {}",
            stats[0], stats[1], stats[2]
        ),
    );
}

fn ignore_game_events(gamesensestub: &server::ServerStub) {
    gamesensestub.expect_request("/multiple_game_events", "");
}
