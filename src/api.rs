use leptos::web_sys;

use crate::{
    models::{
        AstronObjectQueryParams, AstronObjectResponse,
        SearchQueryParams, SearchResponse
    }, errors::AppError
};


fn base_url() -> String {
    web_sys::window().unwrap().location().origin().unwrap()
}


pub async fn search(query: SearchQueryParams) -> Result<SearchResponse, AppError>
{
    let url = format!("{}/search", base_url());
    let client = reqwest::Client::new();
    let res = client.get(url)
        .query(&query)
        .send()
        .await
        .map_err(|e| AppError::FetchError(e.to_string()))?
        .json::<SearchResponse>()
        .await
        .map_err(|e| AppError::JsonError(e.to_string()))?;
    Ok(res)
}


pub async fn get_astron_object_data(query: AstronObjectQueryParams) -> Result<AstronObjectResponse, AppError>
{
    let url = format!("{}/get_astron_object_data", base_url());
    let client = reqwest::Client::new();
    let res = client.get(url)
        .query(&query)
        .send()
        .await
        .map_err(|e| AppError::FetchError(e.to_string()))?
        .json::<AstronObjectResponse>()
        .await
        .map_err(|e| AppError::JsonError(e.to_string()))?;
    Ok(res)
}