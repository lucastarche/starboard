use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use chrono::NaiveDateTime;
use egui::RichText;
use utils::{Gadget, GadgetFactory, MutexExt};
use wttr::WeatherResponse;

mod wttr;

pub struct WeatherGadget {
    id: usize,
    is_open: bool,
    weather_data: Arc<Mutex<WeatherData>>,
}

#[derive(Debug, Default, Clone)]
struct WeatherData {
    temperature: f64,
    feels_like: f64,
    location: String,
    retrieved_at: String,
}

pub struct WeatherGadgetFactory;

#[derive(serde::Deserialize, Default)]
struct WeatherConfig {
    locations: Vec<String>,
}

impl Gadget for WeatherGadget {
    fn id(&self) -> &'static str {
        "weather"
    }

    fn render(&mut self, ctx: &egui::Context) {
        let WeatherData {
            temperature,
            feels_like,
            location,
            retrieved_at,
        } = self.weather_data.locked().clone();

        egui::Window::new("Weather")
            .resizable(false)
            .id(self.make_id(self.id))
            .open(&mut self.is_open)
            .show(ctx, |ui| {
                ui.label(RichText::new(format!("{temperature}°C | {feels_like}°C")).size(64.0));

                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label(RichText::new(format!("{location} (via ")).size(16.0));
                    ui.hyperlink_to(RichText::new("wttr.in").size(16.0), "https://wttr.in");
                    ui.label(RichText::new(format!(" at {retrieved_at})")).size(16.0));
                });
            });
    }

    fn is_open(&self) -> bool {
        self.is_open
    }
}

impl GadgetFactory for WeatherGadgetFactory {
    fn gadget_name(&self) -> &'static str {
        "Weather"
    }

    fn make_gadget(
        &self,
        network_runtime: &utils::NetworkRuntime,
        egui_ctx: &egui::Context,
        id: usize,
    ) -> Box<dyn Gadget> {
        let weather_gadget = WeatherGadget {
            id,
            is_open: true,
            weather_data: Arc::new(Mutex::new(WeatherData::default())),
        };

        let weather_data_lock = weather_gadget.weather_data.clone();
        let ctx = egui_ctx.clone();
        let config: WeatherConfig = utils::config_for_gadget(&weather_gadget).unwrap_or_default();
        network_runtime.spawn(async move {
            loop {
                // FIXME: Allow the user to choose which of the configured locations to open
                let location = match config.locations.get(0) {
                    Some(location) => location,
                    None => "",
                };
                let weather_data = fetch_weather_data(location).await;

                match weather_data {
                    Ok(weather_data) => *weather_data_lock.locked() = weather_data,
                    Err(error) => {
                        println!("Failed to retrieve the weather data from wttr.in: {error}");
                    }
                }

                ctx.request_repaint();
                tokio::time::sleep(Duration::from_secs(5 * 60)).await; // 5 minutes
            }
        });

        Box::new(weather_gadget)
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
