use super::SteelLibError;

pub struct ServerConnector {
    address: String,
}

impl super::EventSender for ServerConnector {
    fn send<Event>(&self, event: Event) -> Result<(), SteelLibError>
    where
        Event: super::Event,
    {
        let mut writer = Vec::new();
        let body = event.body();
        let bytes = body.as_bytes();

        let mut headers = http_req::response::Headers::new();
        headers.insert("User-Agent", "league_of_steel");
        headers.insert("Content-Type", "application/json");
        headers.insert("Host", &self.address);
        headers.insert("Content-Length", &bytes.len());
        headers.insert("Connection", "Close");

        let res = http_req::request::Request::new(&self.get_url(&event.endpoint()))
            .method(http_req::request::Method::POST)
            .headers(headers)
            .body(bytes)
            .send(&mut writer);

        log::debug!("{:?}", res);
        if let Ok(res) = res {
            if res.status_code().is_success() {
                log::debug!("{:?}", writer);
                Ok(())
            } else {
                Err(SteelLibError::SentError(
                    "Unable to read response!".to_string(),
                ))
            }
        } else {
            Err(SteelLibError::SentError(
                "Unable to send event!".to_string(),
            ))
        }
    }
}

impl ServerConnector {
    pub fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
        }
    }

    fn get_url(&self, endpoint: &str) -> http_req::uri::Uri {
        let url = format!("http://{}/{}", self.address, endpoint);
        url.parse().expect("Wrong Url! Wrong address or endpoint!")
    }
}
