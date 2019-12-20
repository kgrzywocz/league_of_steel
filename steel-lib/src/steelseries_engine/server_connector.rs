use super::SteelLibError;

pub struct ServerConnector {
    address: String,
    client: reqwest::Client,
}

impl super::EventSender for ServerConnector {
    fn send<Event>(&self, event: Event) -> Result<(), SteelLibError>
    where
        Event: super::Event,
    {
        let res = self
            .client
            .post(self.get_url(&event.endpoint()))
            .body(event.body())
            .header(reqwest::header::USER_AGENT, "league_of_steel")
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .send();

        log::debug!("{:?}", res);
        if let Ok(mut res) = res {
            if let Ok(text) = res.text() {
                log::debug!("{}", text);
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
            client: reqwest::Client::new(),
        }
    }

    fn get_url(&self, endpoint: &str) -> reqwest::Url {
        let url = format!("http://{}/{}", self.address, endpoint);
        reqwest::Url::parse(&url).expect("Wrong Url! Wrong address or endpoint!")
    }
}
