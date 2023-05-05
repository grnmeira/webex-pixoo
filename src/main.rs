use divoom::*;
use clap::Parser;
use webex;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short = 'i', help = "Webex Integration (Client) ID")]
    integration_client_id: &str,
    #[arg(short = 's', help = "Webex Integration Client Secret")]
    integration_client_secret: &str,
    #[arg(short = 'd', help = "Webex device ID")]
    device_id: Option<&str>
}

#[tokio::main]
async fn main() {
    let divoom = DivoomServiceClient::new();
    let devices = divoom.get_same_lan_devices().await.expect("error getting same lan devices");

    let device = devices.first().expect("no device found");

    println!("{:?}", device);
    
    let client = PixooClient::new(device.device_private_ip.as_str()).expect("not able to connect to device");

    let channel = client.get_current_channel().await.expect("error obtaining device channel");

    println!("{:#?}", channel);

    let gif_url = "https://opengameart.org/sites/default/files/styles/medium/public/kaczuha_1.gif";
    client.play_gif_file(DivoomFileAnimationSourceType::Url, gif_url.to_string()).await.expect("not able to play gif");
}
