fn main() {
    let src_files = std::fs::read_dir("cpp/src")
        .unwrap()
        .map(|f| f.unwrap().path())
        .filter(|f| f.extension().unwrap() == "cpp");

    cc::Build::new()
        .cpp(true)
        //.warnings_into_errors(true)
        .include("cpp/inc")
        .include("backend/interface")
        .files(src_files)
        .compile("lollib");


    let src_files = std::fs::read_dir("backend/dx9/src")
        .unwrap()
        .map(|f| f.unwrap().path())
        .filter(|f| f.extension().unwrap() == "cpp");

    cc::Build::new()
        .cpp(true)
        //.warnings_into_errors(true)
        .include("backend/interface")
        .include("backend/dx9/inc")
        .files(src_files)
        .compile("backend_dx9");
}
