pub struct LolConfig {}

impl LolConfig {
    pub fn create_with_hud_scale(global_scale: &str) -> Self {
        use std::io::Write;

        std::fs::create_dir_all("../target/Config").expect("Unable to create config dir");
        let mut file = std::fs::File::create("../target/Config/game.cfg")
            .expect("Unable to create config game.cfg");
        file.write_all(
            format!(
                r#"
[General]
Colors=32
Height=720
Width=1280
CfgVersion=9.24.4052

[HUD]
ShowAllChannelChat=0
GlobalScale={}
MinimapScale=1.0000"#,
                global_scale
            )
            .as_bytes(),
        )
        .expect("Unable to write to game.cfg");

        Self {}
    }
}

impl Drop for LolConfig {
    fn drop(&mut self) {
        std::fs::remove_file("../target/Config/game.cfg").expect("game.cfg cannot be removed");
    }
}
