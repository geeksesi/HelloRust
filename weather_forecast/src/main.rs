use colored::*;
use serde::Deserialize;
use std::io;
extern crate dotenv;
use std::env;

// structu to deserialize the json response from openweather map
#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

// structure to represent the weather description
#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

// structure to represent the main weather parameters
#[derive(Deserialize, Debug)]
struct Main {
    temp: f32,
    pressure: f32,
    humidity: f32,
}

// structure to represent the wind information
#[derive(Deserialize, Debug)]
struct Wind {
    speed: f32,
}

// Function to get the weather infromation from OpenWeatherMap API
fn get_weather(
    city: &str,
    country: &str,
    api_key: &str,
) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={},{}&unit=metric&appid={}",
        city, country, api_key
    );
    let response = reqwest::blocking::get(&url)?.json::<WeatherResponse>()?;
    Ok(response)
}

// function to display the weather information
fn display_weather(response: &WeatherResponse) {
    println!("Weather in {}", response.name.to_string().bold().green());
    println!(
        "Description: {}",
        response.weather[0].description.to_string().bold().green()
    );
    println!(
        "Temperature: {}°C",
        response.main.temp.to_string().bold().green()
    );
    println!(
        "Pressure: {} hPa",
        response.main.pressure.to_string().bold().green()
    );
    println!(
        "Humidity: {}%",
        response.main.humidity.to_string().bold().green()
    );
    println!(
        "Wind Speed: {} m/s",
        response.wind.speed.to_string().bold().green()
    );
}

fn main() {
    dotenv::dotenv().ok();

    let api_key = env::var("API_KEY")
        .expect("API_KEY must be set")
        .to_string();

    println!("Enter the city name: ");
    let mut city = String::new();
    io::stdin().read_line(&mut city).unwrap();
    city = city.trim().to_string();

    println!("Enter the country code: ");
    let mut country = String::new();
    io::stdin().read_line(&mut country).unwrap();
    country = country.trim().to_string();

    let response = get_weather(&city, &country, &api_key).unwrap();
    display_weather(&response);
}
