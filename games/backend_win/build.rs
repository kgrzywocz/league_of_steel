fn main() {
    let src_files = std::fs::read_dir("cpp/src")
        .unwrap()
        .map(|f| f.unwrap().path())
        .filter(|f| f.extension().unwrap() == "cpp");
    cc::Build::new()
        .cpp(true)
        //.warnings_into_errors(true)
        .include("cpp/inc")
        .include("../backend_interface")
        .files(src_files)
        .compile("backend_win");
}
