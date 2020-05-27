fn main() {
    let mut outdir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    outdir.pop();
    outdir.pop();
    outdir.pop();

    if std::env::var("CARGO_CFG_WINDOWS").is_ok() {
        let src = outdir.join("app.exe");
        outdir.pop();
        let mut dst = outdir.join("Game");
        std::fs::create_dir_all(dst.clone()).expect("Could not crate Game dir in target");
        dst = dst.join("League of Legends.exe");

        println!("Coping {:?} -> {:?}", src, dst);
        std::fs::copy(src, dst).expect("Unable to copy test app");
    }
}
