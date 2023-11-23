/**
@author: amir kamalian
@date: 28 sep 2023
 */

use serde::Deserialize;

pub mod weather {

    // UI ACII art style function calls
    pub fn Thunderstorm() -> &'static str {
        let ASCII_thunderstorm: &str = r#"
                   ___(                        )
                   (                          _)
                  (_                       __))
                    ((                _____)
                      (_________)----'
                         _/  /
                        /  _/
                      _/  /
                     / __/
                   _/ /
                  /__/
                 //
                /'
        "#;
        return ASCII_thunderstorm;
    }

    pub fn Drizzle() -> &'static str {
        let ASCII_drizzile: &str = r#"
                          (`  ).                   _
                         (     ).              .:(`  )`.
                       _(       '`.          :(   .    )
                    .=(`(      .   )     .--  `.  (    ) )
                   ((    (..__.:'-'   .+(   )   ` _`  ) )
            `.     `(       ) )       (   .  )     (   )  ._
              )      ` __.:'   )     (   (   ))     `-'.-(`  )
            )  )  ( )       --'       `- __.'         :(      ))
            .-'  (_.'          .')                    `(    )  ))
                              (_  )                     ` __.:'


        "#;
        return ASCII_drizzile;
    }
    pub fn Rain() {

    }
    pub fn Snow() {

    }
    pub fn Fog() {

    }

    pub fn Mist() {

    }

    pub fn Clear() {

    }

    pub fn Clouds() {

    }


}

/** @brief basic functionality to retrieve weather information. */
pub trait IWeather {
    fn getMain(&self) -> &String;
    fn getMainDescription(&self) -> &String;
    fn getTemp(&self) -> &f64;
    fn getFeelsLike(&self) -> &f64;
    fn getPressure(&self) -> &f64;
    fn getHumidity(&self) -> &f64;
    fn getTimeZone(&self) -> &f64;
    fn getCityName(&self) -> &String;

}

/** @brief json response container */
#[derive(Deserialize, Debug)]
pub struct WeatherResponse {
    coord: Coord,
    weather: Weather,
    base: String,
    main: Main,
    visibility: f64,
    wind: Wind,
    clouds: Clouds,
    dt: f64,
    sys: Sys,
    timezone: f64,
    id: f64,
    name: String,
    cod: f64
}

/** function implementations for WeatherResponse. These are
 mostly getters although additional functionality may be added.*/
impl IWeather for WeatherResponse {
    /** he main descriptor that is used to describe the weather */
    fn getMain(&self) -> &String {
        return &self.weather.data.main;
    }

    fn getMainDescription(&self) -> &String {
        return &self.weather.data.description;
    }

    fn getTemp(&self) -> &f64 {
        return &self.main.temp;
    }

    fn getFeelsLike(&self) -> &f64 {
        return &self.main.feels_like;
    }

    fn getPressure(&self) -> &f64 {
        return &self.main.pressure;
    }

    fn getHumidity(&self) -> &f64 {
        return &self.main.humidity;
    }

    fn getTimeZone(&self) -> &f64 {
        return &self.timezone;
    }

    fn getCityName(&self) -> &String {
        return &self.name;
    }

}


#[derive(Deserialize, Debug)]
pub struct Coord {
    lon: f64,
    lat: f64
}

#[derive(Deserialize, Debug)]
pub struct Weather {
    data: WData,
}

#[derive(Deserialize, Debug)]
pub struct WData {
    id: f64,
    main: String,
    description: String,
    icon: String
}

#[derive(Deserialize, Debug)]
pub struct Main {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: f64,
    humidity: f64
}

#[derive(Deserialize, Debug)]
pub struct Wind {
    speed: f64,
    deg: f64
}

#[derive(Deserialize, Debug)]
pub struct Clouds {
    all: f64
}

#[derive(Deserialize, Debug)]
pub struct Sys {
    r#type: f64,
    id: f64,
    country : String,
    sunrise: f64,
    sunset: f64
}




