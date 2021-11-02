use serde_json::value::Value;

#[tokio::main]
async fn main() {
    let geocode_uri = format!("https://api.mapbox.com/geocoding/v5/mapbox.places/{address}.json?access_token=pk.eyJ1IjoibWRtdW5kbyIsImEiOiJjazdkYmUwOGgxbnBlM2ZudTU0ajM1OHhrIn0.hOVUTlxOg_sIww36kjeBiw&limit=1", address="universidade federal do tocantins");

    let geocode_res = reqwest::get(geocode_uri)
        .await
        .expect("Request error")
        .json::<Value>()
        .await
        .expect("Serialization error");

    let latitude = &geocode_res["features"][0]["center"][1];
    let longitude = &geocode_res["features"][0]["center"][0];
    let location = &geocode_res["features"][0]["place_name"];

    let forecast_uri = format!("https://api.darksky.net/forecast/21921129f2412a7aed8765a17f75fc6c/{lat},{lon}?units=ca&exclude=minutely,hourly,daily,alerts,flags", lat="-10.327262",lon="-48.292613");

    println!("{:#?}", geocode_res);
    // println!("{}", resp["data"][0]["id"]);
}
