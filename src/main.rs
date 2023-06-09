use clap::{command, Parser, Subcommand};
use divoom::*;
use std::time::Duration;
use webex::{self, dto::Data::SubscriptionUpdate};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Run status integration with Pixoo Device")]
    Run {
        #[arg(short = 'c', help = "Webex integration client ID")]
        integration_client_id: String,
        #[arg(short = 's', help = "Webex integration secret")]
        integration_secret: String,
        #[arg(
            short = 'd',
            help = "Webex device ID (if not provided, a new Webex device will be created)"
        )]
        webex_device_id: Option<String>,
        #[arg(
            short = 'p',
            help = "ID of Divoom Pixoo device (can be supressed if there's only a single device present in the local network)"
        )]
        pixoo_device_id: Option<String>,
        #[arg(help = "Path to 64x64 pixels GIF file used when in meeting")]
        gif_in_meeting: String,
        #[arg(help = "Path to 64x64 pixels GIF file used when available")]
        gif_available: String,
    },
    #[command(about = "List Divoom Pixoo devices available in the local network")]
    ListPixooDevices,
}

async fn list_pixoo_devices_on_screen() {
    println!("Looking for Divoom devices...");
    let divoom = DivoomServiceClient::new();
    let devices = divoom
        .get_same_lan_devices()
        .await
        .expect("error getting same lan devices");
    if devices.is_empty() {
        println!("No devices found.");
    } else {
        for device in devices {
            println!(
                "Device ID: {} | Device Name: {} | Device IP: {}",
                device.device_id, device.device_name, device.device_private_ip
            )
        }
    }
}

async fn update_pixoo_from_user_status(
    pixoo_client: &PixooClient,
    in_meeting_gif: &str,
    available_gif: &str,
    status: &str,
) {
    match status {
        "meeting" => {
            pixoo_client
                .render_gif_as_animation(64, Duration::from_millis(100), in_meeting_gif)
                .await
                .expect("not able to play gif");
            println!("Status changed to \"IN MEETING\"");
        }
        _ => {
            pixoo_client
                .render_gif_as_animation(64, Duration::from_millis(100), available_gif)
                .await
                .expect("not able to play gif");
            println!("Status changed to \"AVAILABLE\"");
        }
    }
}

async fn run(
    integration_client_id: &str,
    integration_secret: &str,
    pixoo_device_id: Option<&str>,
    webex_device_id: Option<&str>,
    gif_in_meeting: &str,
    gif_available: &str,
) {
    let divoom_client = DivoomServiceClient::new();

    let divoom_devices = divoom_client
        .get_same_lan_devices()
        .await
        .unwrap_or_else(|e| {
            println!("Error while looking for Divoom devices: {}", e);
            std::process::exit(1);
        });

    println!("Looking for Divoom devices...");

    let divoom_device = if let Some(device_id) = pixoo_device_id {
        divoom_devices
            .iter()
            .find(|d| d.device_id.to_string() == device_id)
            .unwrap_or_else(|| {
                println!("No Divoom device found with ID {}", device_id);
                std::process::exit(1);
            })
    } else {
        divoom_devices.first().unwrap_or_else(|| {
            println!("No Pixoo device found!");
            std::process::exit(1);
        })
    };

    println!(
        "Pixoo device found: {}, IP address: {}",
        divoom_device.device_name, divoom_device.device_private_ip
    );

    let webex_authenticator =
        webex::auth::DeviceAuthenticator::new(integration_client_id, integration_secret);

    let verification_token = webex_authenticator.verify().await.unwrap_or_else(|error| {
        // TODO: webex::auth crate needs to implement Display for Error
        println!("Error obtaining verification token: {:#?}", error);
        std::process::exit(1);
    });

    println!(
        "Please access the following URL to authenticate your device: {}",
        verification_token.verification_uri_complete
    );

    let bearer_token = webex_authenticator
        .wait_for_authentication(&verification_token)
        .await
        .unwrap_or_else(|error| {
            // TODO: webex::auth crate needs to implement Display for Error
            println!("Failure authenticating: {:#?}", error);
            std::process::exit(1);
        });

    println!("Webex authentication succeeded!");

    let webex_client = webex::api::Client::new(&bearer_token, webex_device_id);
    let mut event_listener = webex_client
        .listen_to_events()
        .await
        .unwrap_or_else(|error| {
            println!("Error trying to listen to Webex events: {:#?}", error);
            std::process::exit(1);
        });

    println!("Connecting to Pixoo device...");

    let pixoo_client = PixooClient::new(divoom_device.device_private_ip.as_str())
        .expect("not able to connect to Pixoo device");

    println!("Running...");

    let user_details = webex_client
        .get_my_own_details()
        .await
        .expect("not able to get user's own details");

    update_pixoo_from_user_status(
        &pixoo_client,
        gif_in_meeting,
        gif_available,
        user_details.status.as_str(),
    )
    .await;

    loop {
        let event = event_listener.next().await.unwrap();

        let Some(event_data) = event.data else {
            continue;
        };

        let status = match event_data {
            SubscriptionUpdate {
                subject: _,
                category: _,
                status,
            } => status,
            _ => None,
        };

        if let Some(status) = status {
            update_pixoo_from_user_status(
                &pixoo_client,
                gif_in_meeting,
                gif_available,
                status.as_str(),
            )
            .await;
        }
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::ListPixooDevices => {
            list_pixoo_devices_on_screen().await;
        }
        Commands::Run {
            integration_client_id,
            integration_secret,
            pixoo_device_id,
            webex_device_id,
            gif_in_meeting,
            gif_available,
        } => {
            run(
                integration_client_id.as_str(),
                integration_secret.as_str(),
                pixoo_device_id.as_ref().map(|s| s.as_str()),
                webex_device_id.as_ref().map(|s| s.as_str()),
                gif_in_meeting.as_str(),
                gif_available.as_str(),
            )
            .await;
        }
    }
}
