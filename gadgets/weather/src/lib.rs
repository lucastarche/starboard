use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use chrono::NaiveDateTime;
use egui::RichText;
use utils::{Gadget, MutexExt};
use wttr::WeatherResponse;

mod wttr;

pub struct WeatherGadget {
    weather_data: Arc<Mutex<WeatherData>>,
}

#[derive(Debug, Default, Clone)]
struct WeatherData {
    temperature: f64,
    feels_like: f64,
    location: String,
    retrieved_at: String,
}

impl Gadget for WeatherGadget {
    fn new(network_runtime: &utils::NetworkRuntime, egui_ctx: &egui::Context) -> Self {
        let this = Self {
            weather_data: Arc::new(Mutex::new(Default::default())),
        };

        let weather_data_lock = this.weather_data.clone();
        let ctx = egui_ctx.clone();
        network_runtime.spawn(async move {
            loop {
                // TODO: Allow configuring places in a config file or some such
                let weather_data = fetch_weather_data("").await;

                match weather_data {
                    Ok(weather_data) => *weather_data_lock.locked() = weather_data,
                    Err(error) => {
                        println!("Failed to retrieve the weather data from wttr.in: {error}")
                    }
                }

                ctx.request_repaint();
                tokio::time::sleep(Duration::from_secs(5 * 60)).await; // 5 minutes
            }
        });

        this
    }

    fn render(&mut self, ctx: &egui::Context) {
        let WeatherData {
            temperature,
            feels_like,
            location,
            retrieved_at,
        } = self.weather_data.locked().clone();

        egui::Window::new("Weather").show(ctx, |ui| {
            ui.label(RichText::new(format!("{temperature}°C | {feels_like}°C")).size(64.0));

            ui.horizontal_wrapped(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label(RichText::new(format!("{location} (via ")).size(16.0));
                ui.hyperlink_to(RichText::new("wttr.in").size(16.0), "https://wttr.in");
                ui.label(RichText::new(format!(" at {retrieved_at})")).size(16.0));
            });
        });
    }
}

async fn fetch_weather_data(query: &str) -> anyhow::Result<WeatherData> {
    let response = reqwest::get(format!("https://wttr.in/{query}?format=j1"))
        .await?
        .text()
        .await?;

    let weather_response: WeatherResponse = serde_json::from_str(&response)?;
    let location = format!(
        "{area}, {country}",
        area = weather_response.nearest_area.area_name,
        country = weather_response.nearest_area.country
    );
    let retrieved_at = NaiveDateTime::parse_from_str(
        &weather_response.current_condition.last_update_local_time,
        "%Y-%m-%d %I:%M %p",
    )?
    .time()
    .format("%H:%M")
    .to_string();

    Ok(WeatherData {
        temperature: weather_response.current_condition.temperature,
        feels_like: weather_response.current_condition.feels_like,
        location,
        retrieved_at,
    })
}
