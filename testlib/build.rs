fn main() {
    let mut outdir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    outdir.pop();
    outdir.pop();
    outdir.pop();

    let src = outdir.join("app.exe");
    outdir.pop();
    let mut dst = outdir.join("Game");
    std::fs::create_dir_all(dst.clone());
    dst = dst.join("League of Legends.exe");

    println!("Coping {:?} -> {:?}", src, dst);
    std::fs::copy(src, dst).expect("Unable to copy test app");
}
