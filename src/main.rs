use std::io;
use dotenv::dotenv;
use std::env;
use colored::*;
use serde::Deserialize;

// Struct to deserialize JSON response from openWeatherMap API
#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

// Struct to represent weather desc
#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

// Struct to represent main weather parameters
#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: f64,
    pressure: f64,
}

// Struct to represent wind info
#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

// Function to get weather info from OpenWeatherMap API
fn get_weather_info(
    api_key: &str,
    city: &str,
    country: &str,
) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country, api_key
    );
    let response = reqwest::blocking::get(&url)?;
    let response_json = response.json::<WeatherResponse>()?;
    Ok(response_json)
}

// Function to display the weather info
fn display_weather_info(response: &WeatherResponse) {
    // Extract weather info from response
    let description = &response.weather[0].description;
    let temperature = response.main.temp;
    let humidity = &response.main.humidity;
    let pressure = &response.main.pressure;
    let wind_speed = &response.wind.speed;

    // Formatting info into string
    let weather_text = format!(
        "Weather in {}: {} {}
    > Temperature: {:.1}°C,
    > Humidity: {:.1}%,
    > Pressure: {:.1} hPa,
    > Wind Speed: {:.1} m/s",
        response.name,
        description,
        get_temp_emoji(temperature),
        temperature,
        humidity,
        pressure,
        wind_speed,
    );

    // Coloring the weather text based on weather
    let weather_text_colored = match description.as_str() {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => {
            weather_text.dimmed()
        }
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
        _ => weather_text.normal(),
    };
    // Print colored weather info
    println!("{}", weather_text_colored)
}

// Function to get emoji based on temperature
fn get_temp_emoji(temperature: f64) -> &'static str {
    if temperature < 0.0 {
        "❄️"
    } else if temperature >= 0.0 && temperature < 10.0 {
        "☁️"
    } else if temperature >= 10.0 && temperature < 20.0 {
        "⛅"
    } else if temperature >= 20.0 && temperature < 30.0 {
        "🌤️"
    } else {
        "☀️"
    }
}

fn main() {
    
    dotenv().ok();

    println!("{}", "Welcome to Weather Station!".bright_yellow());
    loop {
        // Country
        println!("{}", "Please enter the name of the country:".bright_green());
        let mut country = String::new();
        io::stdin()
            .read_line(&mut country)
            .expect("Failed to read input!");
        let country: &str = country.trim();

        // City
        println!("{}", "Please enter the name of the city:".bright_green());
        let mut city = String::new();
        io::stdin()
            .read_line(&mut city)
            .expect("Failed to read input!");
        let city: &str = city.trim();

        // API
        let api_key = env::var("OPENWEATHER_API_KEY").expect("Open Weather Api Key not found in .env file");

        // Calling the function to fetch weather info
        match get_weather_info(&api_key, &city, &country) {
            Ok(response) => {
                display_weather_info(&response);
            }
            Err(err) => {
                eprintln!("Error {}", err);
            }
        }

        println!(
            "{}",
            "Do you want to search for weather in another city? (yes/no):".bright_green()
        ); // Prompting user to continue or exit
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input"); // Reading user input for continuation
        let input = input.trim().to_lowercase();

        if input != "yes" && input != "y" {
            println!("Thank you for using !");
            break; // Exiting the loop if user doesn't want to continue
        }
    }
}
