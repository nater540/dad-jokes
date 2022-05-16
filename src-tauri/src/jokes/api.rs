#![allow(dead_code)]

use anyhow::*;
use std::borrow::Cow;
use reqwest::header::{HeaderMap, ACCEPT};
use serde::{de::DeserializeOwned, Serialize};

use crate::jokes::icanhazdadjoke::*;

#[derive(Debug)]
pub struct Client {
  http_client: reqwest::Client,
  endpoint: String
}

/// It builds clients - fairly self explanatory I'd hope.
#[derive(Debug)]
pub struct ClientBuilder<'a> {
  endpoint: Option<Cow<'a, str>>
}

impl<'a> Default for ClientBuilder<'a> {
  fn default() -> ClientBuilder<'a> {
    ClientBuilder {
      endpoint: Some(Cow::Borrowed("https://icanhazdadjoke.com"))
    }
  }
}

impl<'a> ClientBuilder<'a> {
  #[inline]
  pub fn endpoint<S>(&mut self, endpoint: S) -> &mut Self
  where S: Into<Cow<'a, str>> {
    self.endpoint = Some(endpoint.into());
    self
  }

  /// Consumes the builder & creates a new client.
  pub fn create(&self) -> Result<Client> {
    let endpoint = match self.endpoint {
      Some(ref ep) => ep.to_owned().to_string(),
      None         => return Err(anyhow!("Must specify `endpoint`".to_string()))
    };

    let http_client = reqwest::Client::new();
    Ok(Client { http_client, endpoint })
  }
}

impl Client {
  pub fn builder<'a>() -> ClientBuilder<'a> {
    ClientBuilder::default()
  }

  /// Fetches one random joke - for great justice!
  pub async fn random(&self) -> Result<SuccessfulResponse> {
    Ok(self.get(&self.endpoint, None).await?)
  }

  /// Fetches a joke via its id.
  pub async fn fetch<'a, N>(&self, joke_id: N) -> Result<Joke>
  where N: Into<&'a str> {
    let url = format!("{}/j/{}", self.endpoint, joke_id.into());
    Ok(self.get(&url, None).await?)
  }

  pub async fn search<'a, N>(&self, term: N, limit: Option<i32>, page: Option<i32>) -> Result<SearchResponse>
  where N: Into<&'a str> {
    // Limit cap = 30
    let limit = limit
      .and_then(|val| Some(val.clamp(1, 30)))
      .unwrap_or(20);

    let url = format!("{}/search?term={}&limit={}&page={}",
      self.endpoint,
      term.into(),
      limit,
      page.unwrap_or(1)
    );

    Ok(self.get(&url, None).await?)
  }

  /// Helper function to perform a GET request with JSON deserialization.
  async fn get<T: DeserializeOwned>(&self, url: &str, params: Option<Vec<(&str, &str)>>) -> Result<T> {
    let res = self
      .http_client
      .get(url)
      .headers(self.default_headers()?)
      .query(&params)
      .send()
      .await?;

    if res.status().is_success() {
      Ok(res.json::<T>().await?)
    } else {
      Err(anyhow!("Failed to make joke request... sad face"))
    }
  }

   /// Builds a set of default headers for all requests.
   fn default_headers(&self) -> Result<HeaderMap> {
    let mut headers = HeaderMap::new();
    headers.insert(ACCEPT, "application/json".parse()?);
    Ok(headers)
  }
}
