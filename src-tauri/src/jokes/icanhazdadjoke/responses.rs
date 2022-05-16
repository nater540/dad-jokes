use serde::Deserialize;

/// Represents a successful joke response.
#[derive(Deserialize, Debug, Clone)]
pub struct SuccessfulResponse {
  pub id:     String,
  pub status: i32,

  #[serde(flatten)]
  pub joke: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct SearchResponse {
    pub current_page: i32,
    pub limit: i32,
    pub next_page: i32,
    pub previous_page: i32,
    pub results: Vec<Joke>,
    pub search_term: String,
    pub status: i32,
    pub total_jokes: i32,
    pub total_pages: i32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Joke {
  pub id:   String,
  pub joke: String
}
