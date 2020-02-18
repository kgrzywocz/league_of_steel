pub struct ServerStub {
    server: tiny_http::Server,
}

const CORE_PROPS_FILE: &str = "C:\\ProgramData\\SteelSeries\\SteelSeries Engine 3\\coreProps.json";

impl ServerStub {
    pub fn new() -> Self {
        let res = Self { server: tiny_http::Server::http("127.0.0.1:51248").unwrap() };

        use std::io::Write;

        std::fs::create_dir_all("C:\\ProgramData\\SteelSeries\\SteelSeries Engine 3")
            .expect("Unable to create coreProps config dir");
        let mut file =
            std::fs::File::create(CORE_PROPS_FILE).expect("Unable to create coreProps.json");
        file.write_all(r#"{ "address": "127.0.0.1:51248" }"#.as_bytes())
            .expect("Unable to write to coreProps.json");

        res
    }

    pub fn expect_request(&self, url: &str, body_regex: &str) {
        let mut request = self.server.recv().expect("Unable to receive request");
        assert_eq!(url, request.url());
        let mut content = String::new();
        request
            .as_reader()
            .read_to_string(&mut content)
            .expect("Unable to read request");
        let re = regex::RegexBuilder::new(body_regex)
            .multi_line(true)
            .dot_matches_new_line(true)
            .build()
            .expect("Bad regex");
        assert!(
            re.is_match(&content),
            format!(
                "Request:{} with {} did not contain: {}",
                url, content, body_regex
            )
        );

        let response = tiny_http::Response::from_string("OK");
        request.respond(response).expect("Unable to send response");
    }
    pub fn ignore_request(&self) {
        let request = self.server.recv().expect("Unable to receive request");
        let response = tiny_http::Response::from_string("OK");
        request.respond(response).expect("Unable to send response");
    }
}

impl Drop for ServerStub {
    fn drop(&mut self) {
        std::fs::remove_file(CORE_PROPS_FILE).expect("Unable to remove coreProps.json");
    }
}
