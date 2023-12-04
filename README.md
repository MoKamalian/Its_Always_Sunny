# Terminal Weather app made in Rust 

## Description

This is a terminal weather app made in Rust. The program is meant to be used within the terminal  
and uses OpenWeatherMap API to gather weather information on a specified city by the user. The  
program prompts the user for a city name as input. Once the City name has been entered, ASCII  
art is printed followed by general weather information about the city. 

## Getting Started

### Dependencies

* reqwest 0.11.22
* tokio 1
* serde 1.0.189

### Executing program

* First, clone the repo 
* You will need to have rust installed 
* Cargo.toml should have the following dependencies at the minimum in order to work 
#### Cargo.toml
```
[dependencies]
reqwest = { version = "0.11.22", features = ["json"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0.189", features = ["derive"]}
```
* once the dependencies have been downloaded, ensure your own API key is added in ``main.rs``:
```rust
...
let mut api_string: String = String::from("https://api.openweathermap.org/data/2.5/weather?q=");
api_string.push_str(&*input_city);
api_string.push_str("&appid=");
let key = "YOUR API KEY HERE";
api_string.push_str(key);

let http_response = reqwest::get(api_string).await?;
let json_response = http_response.json::<WeatherResponse>().await?;
...
```
* The API key can be obtained from [OpenWeatherMap](https://openweathermap.org/). 
* You should then be able to run the program, input a city name and have the weather information  
returned for that particular city. 


