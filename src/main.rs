use std::io;
use serde::Deserialize;
use colored::*;

// Struct to deserialize json response from OpenWeatherMap API
#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

// Struct to represent weather description
#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

// Struct to represent main weather parameters
#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: f64,
}

// Struct to represent wind information
#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

// function to get weather information from OpenWeatherMap API
fn get_weather_info(city: &str, country_id: &str, key: &str) -> Result<WeatherResponse, reqwest::Error> {
    let url: String = format!("https://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city,
        country_id,
        key);

    let response = reqwest::blocking::get(url)?;
    let response_json: WeatherResponse = response.json::<WeatherResponse>()?;

    Ok(response_json)
    
}

// function to display weather information
fn display_weather_info(response: &WeatherResponse) {
    // extract info from response
    let desc = &response.weather[0].description;
    let temp = response.main.temp;
    let humid = response.main.humidity;
    let wind_speed = response.wind.speed;

    // Formatting weather info to a string
    let weather_info = format!(
        "weather in {}: {}
        > temp: {:.1}C
        > humidity: {:.1}%
        > wind speed: {:.1} m/s
        ",
        response.name,
        desc,
        temp,
        humid,
        wind_speed,
    );

    // Coloring the weather text based on weather conditions
    let weather_info_colored = match desc.as_str() {
        "clear sky" => weather_info.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_info.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => weather_info.dimmed(),
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_info.bright_cyan(),
        _ => weather_info.normal(),
    };

    println!("{}", weather_info_colored);

}

fn main() {
    
    println!("{}", "Welcome to Weather Station".bright_magenta());

    // get API key from user
    println!("{}", "Please supply a valid OpenWeather API key".bright_yellow());
    let mut api_key = String::new();
    io::stdin().read_line(&mut api_key).expect("Failed to read input");
    let api_key = api_key.trim();

    // Main Loop
    loop {
        println!("{}", "Please enter a City Name".bright_magenta());
        let mut city_name = String::new();
        io::stdin().read_line(&mut city_name).expect("Failed to read input");
        let city_name = city_name.trim();

        println!("{}", "Please enter a Country Code".bright_cyan());
        let mut country_code = String::new();
        io::stdin().read_line(&mut country_code).expect("Failed to read input");
        let country_code = country_code.trim();

        match get_weather_info(city_name, country_code, api_key) {
            Ok(response) => {
                display_weather_info(&response);
            }
            Err(err) => {
                eprint!("Error: {}", err);
            }
        }

        println!("{}", "Would you like to check another location? (y/n)".bright_red());
        let mut ans = String::new();
        io::stdin().read_line(&mut ans).expect("Failed to read input");
        let ans = ans.trim();

        match ans {
            "n" | "N" => break,
            _ => continue,
        }
    }
}