fn main() {
    let mut outdir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    outdir.pop();
    outdir.pop();
    outdir.pop();

    let src = outdir.join("app.exe");
    outdir.pop();
    let dst = outdir.join("League of Legends.exe");
    // Riot Games\League of Legends\Game
    // Riot Games\League of Legends\Config\game.cfg

    println!("Moving {:?} -> {:?}", src, dst);
    std::fs::rename(src, dst).expect("Unable to move test app");
}
