use std::{
    thread::{self, sleep},
    time::Duration,
};

use discord_rich_presence::{
    activity::{Activity, Assets, Button},
    DiscordIpc, DiscordIpcClient,
};

#[flutter_rust_bridge::frb]
#[derive(Clone)]
pub struct DiscordButton {
    pub label: String,
    pub url: String,
}

#[flutter_rust_bridge::frb(sync)]
pub fn set_discord_rpc(
    client_id: String,
    large_image_key: String,
    large_text_key: String,
    small_image_key: String,
    small_text_key: String,
    details: String,
    state: String,
    buttons: Vec<DiscordButton>,
) {
    thread::spawn(move || {
        let mut drpc = match DiscordIpcClient::new(client_id.as_str()) {
            Ok(client) => client,
            Err(e) => {
                eprintln!("Error creating Discord client: {}", e);
                return;
            }
        };

        if let Err(e) = drpc.connect() {
            eprintln!("Failed to connect to Discord: {}", e);
            return;
        }

        if buttons.len() >= 2 {
            panic!("Only one or two buttons are allowed")
        }

        loop {
            let btns: Vec<Button> = buttons
                .iter()
                .map(|btn| {
                    let label = &btn.label;
                    let url = &btn.url;
                    Button::new(label, url)
                })
                .collect();

            let activity: Activity = Activity::new();
            let assets = Assets::new();

            if let Err(e) = drpc.set_activity(
                activity
                    .state(&state)
                    .assets(
                        assets
                            .large_image(&large_image_key)
                            .large_text(&large_text_key)
                            .small_image(&small_image_key)
                            .small_text(&small_text_key),
                    )
                    .details(&details)
                    .buttons(btns),
            ) {
                if (e.to_string().contains("Broken pipe")) {
                    eprintln!("Connection lost: {}, attempting to reconnect...", e);
                    if let Err(e) = drpc.reconnect() {
                        eprintln!("Failed to reconnect to Discord: {}", e);
                        return;
                    }
                } else {
                    eprintln!("Failed to set presence: {}", e);
                    return;
                }
                return;
            }

            sleep(Duration::from_secs(30))
        }
    });
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
