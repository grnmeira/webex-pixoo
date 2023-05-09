use clap::{Parser, Subcommand, Args, Arg, ArgGroup, ArgAction, command};
use divoom::*;
use webex;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about="Run status integration with Pixoo Device")]
    Run { 
        #[arg(short='c', help="Webex integration client ID")]
        integration_client_id: String,
        #[arg(short='s', help="Webex integration secret")]
        integration_secret_id: String,
        #[arg(short='d', help="Webex device ID (if not provided, a new Webex device will be created)")]
        webex_device_id: Option<String>,
        #[arg(short='p', help="ID of Divoom Pixoo device (can be supressed if there's only a single device present in the local network)")]
        pixoo_device_id: Option<String>
    },
    #[command(about="List Divoom Pixoo devices available in the local network")]
    ListPixooDevices
}

#[tokio::main]
async fn main() {

    let cli = Cli::parse();

    std::process::exit(1);

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

    panic!("hey");

    // let webex_authenticator = webex::auth::DeviceAuthenticator::new(
    //     &args.integration_client_id,
    //     &args.integration_client_secret,
    // );

    // let verification_token = webex_authenticator.verify().await.unwrap_or_else(|error| {
    //     // TODO: webex crate needs to implement Display for Error
    //     panic!("error obtaining verification token: {:#?}", error);
    // });

    // println!(
    //     "Please access the following URL to authenticate your device: {}",
    //     verification_token.verification_uri_complete
    // );

    // let bearer_token = webex_authenticator
    //     .wait_for_authentication(&verification_token)
    //     .await
    //     .unwrap_or_else(|error| {
    //         // TODO: webex crate needs to implement Display for Error
    //         panic!("failure authenticating: {:#?}", error);
    //     });

    // let webex_client = webex::api::Client::new(&bearer_token, None);
    // let mut event_listener = webex_client
    //     .listen_to_events()
    //     .await
    //     .unwrap_or_else(|error| {
    //         panic!("error trying to listen to Webex events: {:#?}", error);
    //     });

    // loop {
    //     let event = event_listener.next().await.unwrap();
    //     println!("{:#?}", event);
    // }

    // let client =
    //     PixooClient::new(device.device_private_ip.as_str()).expect("not able to connect to device");

    // let channel = client
    //     .get_current_channel()
    //     .await
    //     .expect("error obtaining device channel");

    // println!("{:#?}", channel);

    // let gif_url = "https://opengameart.org/sites/default/files/styles/medium/public/kaczuha_1.gif";
    // client
    //     .play_gif_file(DivoomFileAnimationSourceType::Url, gif_url.to_string())
    //     .await
    //     .expect("not able to play gif");
}
