/**
@author: ammir kamalian
@date: 28 sep 2023
*/

mod utility;

use std::io;
use reqwest;
use std::error::Error;
use utility::WeatherResponse;
use crate::utility::IWeather;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    /** user city input and validation */
    println!("Please input the name of the City: ");
    let mut input_city = String::new();
    io::stdin().read_line(&mut input_city);


    /** @brief API functionality. This function makes the request to OpenWeatherMap
    to fetch weather data. */
    let mut api_string: String = String::new();


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
    println!("{art}", art=utility::weather::Thunderstorm());
    println!("Todays Weather: {main}",main=main);
    println!("City:{city:>width$}\nTemperature: {temp:>width$}\nFeels Like: {fl:>width$.2}\nPressure: {pressure:>width$}\nHumidity: {humidity:>width$}",
             city=city_name,
             temp=temp,
             fl=feels_like,
             pressure=pressure,
             humidity=humidity);

    } else if(main == "Drizzle") {
    // print drizzle art
    println!("{art}", art=utility::weather::Drizzle());
    println!("Todays Weather: {main}",main=main);
    println!("City:{city:>width$}\nTemperature: {temp:>width$}\nFeels Like: {fl:>width$.2}\nPressure: {pressure:>width$}\nHumidity: {humidity:>width$}",
             city=city_name,
             temp=temp,
             fl=feels_like,
             pressure=pressure,
             humidity=humidity);

    } else if(main == "Rain") {
    // print rain art
    println!("{art}", art=utility::weather::Rain());
    println!("Todays Weather: {main}",main=main);
    println!("City:{city:>width$}\nTemperature: {temp:>width$}\nFeels Like: {fl:>width$.2}\nPressure: {pressure:>width$}\nHumidity: {humidity:>width$}",
             city=city_name,
             temp=temp,
             fl=feels_like,
             pressure=pressure,
             humidity=humidity);

    } else if(main == "Snow") {
    // print snow art
    println!("{art}", art=utility::weather::Snow());
    println!("Todays Weather: {main}",main=main);
    println!("City:{city:>width$}\nTemperature: {temp:>width$}\nFeels Like: {fl:>width$.2}\nPressure: {pressure:>width$}\nHumidity: {humidity:>width$}",
             city=city_name,
             temp=temp,
             fl=feels_like,
             pressure=pressure,
             humidity=humidity);


    } else if(main == "Fog") {
    // print fog art
    println!("{art}", art=utility::weather::Fog());
    println!("Todays Weather: {main}",main=main);
    println!("City:{city:>width$}\nTemperature: {temp:>width$}\nFeels Like: {fl:>width$.2}\nPressure: {pressure:>width$}\nHumidity: {humidity:>width$}",
             city=city_name,
             temp=temp,
             fl=feels_like,
             pressure=pressure,
             humidity=humidity);

    } else if(main == "Mist") {
    // print mist art
    println!("{art}", art=utility::weather::Mist());
    println!("Todays Weather: {main}",main=main);
    println!("City:{city:>width$}\nTemperature: {temp:>width$}\nFeels Like: {fl:>width$.2}\nPressure: {pressure:>width$}\nHumidity: {humidity:>width$}",
             city=city_name,
             temp=temp,
             fl=feels_like,
             pressure=pressure,
             humidity=humidity);

    } else if(main == "Clear") {
    // print clear art
    println!("{art}", art=utility::weather::Clear());
    println!("Todays Weather: {main}",main=main);
    println!("City:{city:>width$}\nTemperature: {temp:>width$}\nFeels Like: {fl:>width$.2}\nPressure: {pressure:>width$}\nHumidity: {humidity:>width$}",
             city=city_name,
             temp=temp,
             fl=feels_like,
             pressure=pressure,
             humidity=humidity);

    } else if(main == "Clouds") {
    // print clouds art
    println!("{art}", art=utility::weather::Clouds());
    println!("Todays Weather: {main}",main=main);
    println!("City:{city:>width$}\nTemperature: {temp:>width$}\nFeels Like: {fl:>width$.2}\nPressure: {pressure:>width$}\nHumidity: {humidity:>width$}",
             city=city_name,
             temp=temp,
             fl=feels_like,
             pressure=pressure,
             humidity=humidity);

    } else {
    // all other cases print no ascii art, simply output the  weather information
    println!("Todays Weather: {main}",main=main);
    println!("City:{city:>width$}\nTemperature: {temp:>width$}\nFeels Like: {fl:>width$.2}\nPressure: {pressure:>width$}\nHumidity: {humidity:>width$}",
             city=city_name,
             temp=temp,
             fl=feels_like,
             pressure=pressure,
             humidity=humidity);

    }

    Ok(())
}








