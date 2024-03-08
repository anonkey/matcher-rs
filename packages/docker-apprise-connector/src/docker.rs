use anyhow::{anyhow, Result};
use reqwest::{header::ACCEPT, Client};
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Clone, Debug)]
pub struct AppriseClient {
    client: Client,
    events_url: Url,
}

const EVENTS_URL: &str = "/events";

pub(crate) fn process_into_stream<T>(
    &self,
    req: Result<Request<Full<Bytes>>, Error>,
) -> impl Stream<Item = Result<T, Error>> + Unpin
where
    T: DeserializeOwned,
{
    Box::pin(
        self.process_request(req)
            .map_ok(Docker::decode_into_stream::<T>)
            .into_stream()
            .try_flatten(),
    )
}

impl DockerClient {
    pub fn new(api_url: String) -> Result<Self> {
        let base_url = Url::parse(&api_url)?;

        Ok(AppriseClient {
            client: Client::new(),
            events_url: base_url.join(EVENTS_URL)?,
        })
    }

    pub async fn notify(&self, payload: NotifyPayload) -> Result<()> {
        let key = payload.key.to_owned();
        let body = serde_json::to_value(payload)?;
        println!("sending request to apprise {key} {body}");
        let response = self
            .client
            .get(self.events_url)
            .header(ACCEPT, "application/json")
            .json(&body)
            .send()
            .await?;

        let status = response.status();

        if status == 200 {
            Ok(())
        } else {
            Err(anyhow!(format!("Wrong status code {status}")))
        }
    }
}