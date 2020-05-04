use testlib::*;

fn main() {
    let gamesensestub = server::ServerStub::new();
    loop {
        gamesensestub.ignore_request();
    }
}
