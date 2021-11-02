use serde_json::value::Value;

#[tokio::main]
async fn main() {
    let mut args = std::env::args().skip(1);
    let address = args.next().expect("Address required");

    let geocode_uri = format!(
        "https://api.mapbox.com/geocoding/v5/mapbox.places/{}.json?access_token=pk.eyJ1IjoibWRtdW5kbyIsImEiOiJjazdkYmUwOGgxbnBlM2ZudTU0ajM1OHhrIn0.hOVUTlxOg_sIww36kjeBiw&limit=1",
        address
    );

    let geocode_res = reqwest::get(geocode_uri)
        .await
        .expect("Request error")
        .json::<Value>()
        .await
        .expect("Serialization error");

    let latitude = &geocode_res["features"][0]["center"][1]
        .as_f64()
        .expect("Not a f64");
    let longitude = &geocode_res["features"][0]["center"][0]
        .as_f64()
        .expect("Not a f64");
    let location = &geocode_res["features"][0]["text"]
        .as_str()
        .expect("Not a str");

    let forecast_uri = format!(
        "https://api.darksky.net/forecast/21921129f2412a7aed8765a17f75fc6c/{},{}?units=ca&exclude=minutely,hourly,daily,alerts,flags",
        latitude, longitude
    );

    let forecast_res = reqwest::get(forecast_uri)
        .await
        .expect("Request error")
        .json::<Value>()
        .await
        .expect("Serialization error");

    let summary = &forecast_res["currently"]["summary"]
        .as_str()
        .expect("Not a str");
    let temperature = &forecast_res["currently"]["temperature"]
        .as_f64()
        .expect("Not a f64");
    let precip_probability = &forecast_res["currently"]["precipProbability"]
        .as_f64()
        .expect("Not a f64");
    let precip_type = &forecast_res["currently"]["precipType"]
        .as_str()
        .expect("Not a str");

    println!(
        "At {}, currently is {}, {} degrees and there is a {:.2}% chance of {}.",
        location,
        summary,
        temperature,
        precip_probability * 100.0,
        precip_type
    );
}
