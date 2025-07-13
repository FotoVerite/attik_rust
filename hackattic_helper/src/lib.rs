use reqwest::{Client, Response};
use serde::Serialize;

pub struct HackAtticApi {
    token: String,
    base_url: String,
    challenge: String,
    client: Client,
}

impl HackAtticApi {
    pub fn new(challenge: &str) -> Self {
        Self {
            token: "ad511f7fb5136d19".into(),
            base_url: "https://hackattic.com/challenges/".into(),
            challenge: challenge.into(),
            client: Client::new(),
        }
    }

    pub async fn get_challenge(&self) -> Result<Response, reqwest::Error> {
        let url = format!(
            "{}{}/problem?access_token={}",
            self.base_url, self.challenge, self.token
        );
        self.client.get(&url).send().await
    }

    pub async fn send_solution<T: Serialize>(
        &self,
        solution: &T,
    ) -> Result<Response, reqwest::Error> {
        let url = format!(
            "{}{}/solve?access_token={}",
            self.base_url, self.challenge, self.token
        );
        self.client.post(&url).json(solution).send().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}

// class Api():
//     def __init__(self, challenge):
//         self.token = 'ad511f7fb5136d19'
//         self.base_url = 'https://hackattic.com/challenges/'
//         self.challenge = challenge

//     def get_challenge(self):
//         return requests.get(self.base_url + self.challenge + "/problem?access_token=" + self.token)

//     def send_solution(self, solution):
//           return requests.post(self.base_url + self.challenge + "/solve?access_token=" + self.token, json=solution)
