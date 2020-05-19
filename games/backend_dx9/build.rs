fn main() {
    if std::env::var("CARGO_CFG_WINDOWS").is_ok() {
        let src_files = std::fs::read_dir("cpp")
            .unwrap()
            .map(|f| f.unwrap().path())
            .filter(|f| f.extension().unwrap() == "cpp");
        cc::Build::new()
            .cpp(true)
            //.warnings_into_errors(true)
            .include("../backend_interface")
            .files(src_files)
            .compile("backend");
    }
}
