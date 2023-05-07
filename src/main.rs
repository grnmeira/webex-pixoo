use clap::Parser;
use divoom::*;
use webex;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'i', help = "Webex Integration (Client) ID")]
    integration_client_id: String,
    #[arg(short = 's', help = "Webex Integration Client Secret")]
    integration_client_secret: String,
    #[arg(short = 'd', help = "Webex device ID")]
    device_id: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let webex_authenticator = webex::auth::DeviceAuthenticator::new(
        &args.integration_client_id,
        &args.integration_client_secret,
    );

    let verification_token = webex_authenticator.verify().await.unwrap_or_else(|error| {
        // TODO: webex crate needs to implement Display for Error
        panic!("error obtaining verification token: {:#?}", error);
    });

    println!(
        "Please access the following URL to authenticate your device: {}",
        verification_token.verification_uri_complete
    );

    let bearer_token = webex_authenticator
        .wait_for_authentication(&verification_token)
        .await
        .unwrap_or_else(|error| {
            // TODO: webex crate needs to implement Display for Error
            panic!("failure authenticating: {:#?}", error);
        });

    let webex_client = webex::api::Client::new(&bearer_token, None);
    let mut event_listener = webex_client
        .listen_to_events()
        .await
        .unwrap_or_else(|error| {
            panic!("error trying to listen to Webex events: {:#?}", error);
        });

    loop {
        let event = event_listener.next().await.unwrap();
        println!("{:#?}", event);
    }

    println!("Looking for Divoom devices...");

    let divoom = DivoomServiceClient::new();
    let devices = divoom
        .get_same_lan_devices()
        .await
        .expect("error getting same lan devices");

    let device = devices
        .first()
        .expect("no Divoom device found in local network");

    println!("{:?}", device);

    let client =
        PixooClient::new(device.device_private_ip.as_str()).expect("not able to connect to device");

    let channel = client
        .get_current_channel()
        .await
        .expect("error obtaining device channel");

    println!("{:#?}", channel);

    let gif_url = "https://opengameart.org/sites/default/files/styles/medium/public/kaczuha_1.gif";
    client
        .play_gif_file(DivoomFileAnimationSourceType::Url, gif_url.to_string())
        .await
        .expect("not able to play gif");
}
