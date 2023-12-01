


pub async fn get_astron_object_data(
    name: &str
    lon: f64
    lat: f64
    elevation: f64
    when: NaiveDateTime
) -> Result<AstronObjectResponse, Report>
{
    let res = gloo_net::http::Request::get("get_astron_object_data")
        .query([
            ("name", name),
            ("lon", lon.to_string()),
            ("lat", lat.to_string()),
            ("elevation", elevation.to_string()),
            ("when", when),
        ])
        .send()
        .await?
        .json::<AstronObjectResponse>()
        .await?;
    Ok(res)
}