#[derive(serde::Deserialize)]
struct Slip {
    advice: String,
}

#[derive(serde::Deserialize)]
struct Response {
    slip: Slip,
}

#[derive(Debug)]
struct MainError {
    message: String,
}

impl std::fmt::Display for MainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

#[tokio::main]
async fn main() -> Result<(), MainError> {
    let url: &str = "https://api.adviceslip.com/advice";
    let response: reqwest::Response =
        reqwest::get(url)
            .await
            .map_err(|err: reqwest::Error| MainError {
                message: err.to_string(),
            })?;

    if response.status().is_success() {
        let body: String = response
            .text()
            .await
            .map_err(|err: reqwest::Error| MainError {
                message: err.to_string(),
            })?;

        let parsed: Response =
            serde_json::from_str(&body).map_err(|err: serde_json::Error| MainError {
                message: err.to_string(),
            })?;

        println!("Advice: {}", parsed.slip.advice);
    } else {
        println!("Failed to fetch. Status: {}", response.status());
    }

    Ok(())
}
