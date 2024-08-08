# Weather-Fetching Terminal Application

## Introduction
The Weather-Fetching Terminal Application is a Rust-based command-line tool that retrieves and displays real-time weather information for a specified city. This project leverages the OpenWeatherMap API to provide detailed weather data, including temperature, humidity, pressure, and wind speed. The output is color-coded based on weather conditions for an enhanced user experience.

## Project Overview
This terminal application is designed to offer a quick and efficient way to check the weather directly from the command line. It accepts user input for the city and country, fetches the weather data from the OpenWeatherMap API, and displays the results in a well-formatted, colored output. The application emphasizes simplicity, usability, and the practical use of Rust for interacting with external APIs.

## Detailed Features
- **API Integration**: Connects to the OpenWeatherMap API to fetch real-time weather data for any city in the world.
- **User Interaction**: Prompts the user for city and country inputs to retrieve specific weather information.
- **Colored Output**: Weather conditions are displayed in color, with different conditions such as clear skies, clouds, rain, etc., each having a distinct color for easy identification.
- **Data Display**: Shows temperature, humidity, pressure, wind speed, and weather description in a structured format.
- **Error Handling**: Provides meaningful error messages for invalid inputs or network issues, ensuring a smooth user experience.

## Technologies Used
- **Language**: Rust
- **API**: OpenWeatherMap API
- **Libraries**: `reqwest` for HTTP requests, `serde` for JSON deserialization, `colored` for terminal output coloring, `dotenv` for environment variable management.

## Tech Stack
- **Language**: Rust
- **API**: OpenWeatherMap
- **Dependencies**: `reqwest`, `serde`, `colored`, `dotenv`