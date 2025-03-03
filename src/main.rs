
use anyhow::Result;
use embedded_svc::mqtt::client::{
    Details::Complete, EventPayload::Error, EventPayload::Received, QoS,
};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{
        delay,
        i2c::{I2cConfig, I2cDriver},
        prelude::*,
    },
    mqtt::client::{Details, EspMqttClient, MqttClientConfiguration},
};
use log::info;
use mqtt_messages::hello_topic;
use rgb_led::{RGB8, WS2812RMT};
use shtcx::{self, shtc3, PowerMode};
use types::*;
use std::{thread::sleep, time::Duration};
use wifi::wifi;
use serde::{Serialize, Deserialize};
use serde_json::to_string;
pub mod types; 


const UUID: &str = get_uuid::uuid();

#[toml_cfg::toml_config]
pub struct Config {
    #[default("localhost")]
    mqtt_host: &'static str,
    #[default("")]
    mqtt_user: &'static str,
    #[default("")]
    mqtt_pass: &'static str,
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();


    let event = LogEvent {
        eCode: EventCode::FirmwareUpdate as u16,
        eVal1: 17,  // User level
        eVal2: 102, // Current version
        eVal3: 103, // New version
        eVal4: 0,   // Not used
    };

    let rts = RealtimeStatusWrapper {
        RealtimeStatus: RealtimeStatus {
            fryST: FryingState::Heating as u8,
            prbT: 300,
            prbT10: 3000,
            cjT: 250,
            cjT10: 2500,
            curRcp: 5,
            curGrp: 1,
            curSP: 375,
            curLvl: 16,
        },
    };

    let json = to_string(&rts).unwrap();
    log::info!("json:::{}", json);


    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;

    // The constant `CONFIG` is auto-generated by `toml_config`.
    let app_config = CONFIG;

    // Connect to the Wi-Fi network
    let _wifi = wifi(
        app_config.wifi_ssid,
        app_config.wifi_psk,
        peripherals.modem,
        sysloop,
    )?;

    info!("Our UUID is:");
    info!("{}", UUID);

    let pins = peripherals.pins;
    let sda = pins.gpio10;
    let scl = pins.gpio8;
    let i2c = peripherals.i2c0;
    let config = I2cConfig::new().baudrate(100.kHz().into());
    let i2c = I2cDriver::new(i2c, sda, scl, &config)?;
    let mut temp_sensor = shtc3(i2c);
    let mut delay = delay::Ets;

    let mut led = WS2812RMT::new(pins.gpio2, peripherals.rmt.channel0)?;
    led.set_pixel(RGB8::new(1, 1, 0))?;

    // Client configuration:
    let broker_url = if !app_config.mqtt_user.is_empty() {
        format!(
            "mqtt://{}:{}@{}",
            app_config.mqtt_user, app_config.mqtt_pass, app_config.mqtt_host
        )
    } else {
        format!("mqtt://{}", app_config.mqtt_host)
    };

    let mqtt_config = MqttClientConfiguration::default();

    let mut client = EspMqttClient::new_cb(&broker_url, &mqtt_config, move |_message_event| {
        // ... your handler code here - leave this empty for now
        // we'll add functionality later in this chapter
    })?;

    // client.enqueue(&hello_topic(UUID), QoS::AtLeastOnce, true, payload)?;
    let mut is_green = true; // Track color state
    loop {
        // Toggle between green and blue
        let color = if is_green {
            RGB8::new(0, 255, 0) // Green
        } else {
            RGB8::new(0, 0, 255) // Blue
        };

        // Set LED color
        led.set_pixel(color)?;
        is_green = !is_green; // Flip color state for next iteration

        // Create message with current color
        let message = format!(
            "hello world! Current LED color: {:?}",
            if is_green { "Green" } else { "Blue" }
        );

        // Publish message
        client.enqueue(
            "ESP32 on Luke's Desk",
            QoS::AtLeastOnce,
            false,
            message.as_bytes(),
        )?;

        sleep(Duration::from_secs(1));
    }
    
}
