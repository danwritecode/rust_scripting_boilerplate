use serde::{Serialize, Deserialize};
use reqwest::header::AUTHORIZATION;
use reqwest::header;

#[tokio::main]
async fn main() {

}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateStruct {
    pub foo: String,
}

async fn update_fw_deal_fill_number(id: &i64) -> Result<(), reqwest::Error> {
    let update = UpdateStruct {
        foo: "foo".to_string()
    };

    let url = format!("{}", id);
    let mut headers = header::HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("").parse().unwrap());
    let client = reqwest::Client::new();

    let res = client.put(url)
        .headers(headers)
        .json(&update)
        .send()
        .await?;

    Ok(())
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseStruct {
    pub foo: String,
}

async fn get_day_supply(id: &i64) -> Result<(), reqwest::Error> {
    let url = format!("{}", id);
    let mut headers = header::HeaderMap::new();
    headers.insert(AUTHORIZATION, format!("").parse().unwrap());
    let client = reqwest::Client::new();

    let res = client.get(url)
        .headers(headers)
        .send()
        .await?
        .json::<ResponseStruct>()
        .await?;

    Ok(())
}
