fn main() {
    if std::env::var("CARGO_CFG_WINDOWS").is_ok() {
        let win_common_files = std::fs::read_dir("../backend_win_common/cpp")
            .unwrap()
            .map(|f| f.unwrap().path())
            .filter(|f| f.extension().unwrap() == "cpp");
        let src_files = std::fs::read_dir("cpp")
            .unwrap()
            .map(|f| f.unwrap().path())
            .filter(|f| f.extension().unwrap() == "cpp");
        cc::Build::new()
            .cpp(true)
            //.warnings_into_errors(true)
            .include("../backend_interface")
            .include("cpp")
            .files(win_common_files)
            .files(src_files)
            .compile("backend");
    }
}
