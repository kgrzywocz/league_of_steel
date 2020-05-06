fn main() {
    let src_files = std::fs::read_dir("cpp/src")
        .unwrap()
        .map(|f| f.unwrap().path())
        .filter(|f| f.extension().unwrap() == "cpp");
    cc::Build::new()
        .cpp(true)
        //.warnings_into_errors(true)
        .flag_if_supported("--std=c++14")
        .include("cpp/inc")
        .include("backend_interface")
        .files(src_files)
        .compile("lollib");

    if std::env::var("CARGO_CFG_WINDOWS").is_ok() {
        let src_files = std::fs::read_dir("backend_gdi/cpp")
            .unwrap()
            .map(|f| f.unwrap().path())
            .filter(|f| f.extension().unwrap() == "cpp");
        cc::Build::new()
            .cpp(true)
            //.warnings_into_errors(true)
            .include("backend_interface")
            .files(src_files)
            .compile("backend_gdi");
    }
}
