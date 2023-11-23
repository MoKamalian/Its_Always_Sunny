/**
@author: ammir kamalian
@date: 28 sep 2023
*/

mod utility;

use reqwest;
use std::error::Error;
use utility::WeatherResponse;
use crate::utility::IWeather;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

  /** @brief API functionality. This function makes the request to OpenWeatherMap
  to fetch weather data. */
  let http_response = reqwest::get("https://api.openweathermap.org/data/2.5/weather?q=halifax&appid=")
  .await?;
  let json_response = http_response.json::<WeatherResponse>().await?;

  //lambda for converting between K to C
  let closure_convertT: fn(&f64) -> f64 = |i: &f64| -> f64 { i - 273.15 } ;

  // weather information
  let city_name: &String = json_response.getCityName();
  let description: &String = json_response.getMainDescription();
  let main: &String = json_response.getMain();
  let temp: f64 = closure_convertT(json_response.getTemp());
  let feels_like: f64 = closure_convertT(json_response.getFeelsLike());
  let pressure: &f64 = json_response.getPressure();
  let humidity: &f64 = json_response.getHumidity();


  // for formatting for txt being printed
  let width: usize = 8;


  if(main == "Thunderstorm") {
   // print thunderstorm art
   utility::weather::Thunderstorm();
   println!("\nCity:{city:>width$}\n
             Temperature:{temp:>width$}\n
             Feels Like:{fl:>width$}\n
             Pressure:{pressure:>width$}\n
             Humidity:{humidity:>width$}",
            city=city_name,
            temp=temp,
            fl=feels_like,
            pressure=pressure,
            humidity=humidity);

  } else if(main == "Drizzle") {
    // print drizzle art
   utility::weather::Drizzle();

  } else if(main == "Rain") {
    // print rain art
   utility::weather::Rain();

  } else if(main == "Snow") {
    // print snow art
   utility::weather::Snow();

  } else if(main == "Fog") {
    // print fog art
   utility::weather::Fog();

  } else if(main == "Mist") {
    // print mist art
   utility::weather::Mist();
   println!("\nCity: {city:>width$}\n
             Temperature: {temp:>width$}\n
             Feels Like: {fl:>width$}\n
             Pressure: {pressure:>width$}\n
             Humidity: {humidity:>width$}",
            city=city_name,
            temp=temp,
            fl=feels_like,
            pressure=pressure,
            humidity=humidity);

  } else if(main == "Clear") {
    // print clear art
   utility::weather::Clear();

  } else if(main == "Clouds") {
    // print clouds art
   utility::weather::Clouds();
   println!("\nCity:{city:>width$}\n
             Temperature:{temp:>width$}\n
             Feels Like:{fl:>width$}\n
             Pressure:{pressure:>width$}\n
             Humidity:{humidity:>width$}",
            city=city_name,
            temp=temp,
            fl=feels_like,
            pressure=pressure,
            humidity=humidity);

  } else {
    // all other cases print nothing

  }

  Ok(())
}








