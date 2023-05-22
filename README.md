# Webex/Pixoo Integration
Are you tired of working from home and being interrupted during calls by people that live with you? This Webex/Pixoo integration is the solution. This binary crate provides an integration between [Webex](www.webex.com) and [Divoom Pixoo-64](https://divoom.com/products/pixoo-64), wich will display different GIF animations depending on your Webex status (in meeting or available). All that in a (blazingly fast and memory safe)™️ way.
# Usage
This is a binary crate and can easily be compiled using `cargo build` and it provides a CLI interface for use. It will also require a Webex Integration, which can be created in the [Webex for developers](https://developer.webex.com/docs/integrations) portal. Once the integration is created, you'll need its "client ID" and "client secret", both should be provided by the Webex portal for integrations in the "OAuth Settings" for your Webex Integration.
## Listing Pixoo devices
In order to list Pixoo devices available in your local network, you can use:
```
$ webex-pixoo list-pixoo-devices 
```
If any devices are available, it should show something similar to:
```
Looking for Divoom devices...
Device ID: <Device ID> | Device Name: Pixoo64 | Device IP: <Device IP>
```
## Displaying meeting status
In order to display your status using the Pixoo device you should use the `run` command as follows:
```
$ webex-pixoo run -c <Your Webex Integration ID> -s <Your Webex Integration Secret> <Path to your "IN MEETING" GIF> <Path to your "AVAILABLE" GIF>
```
# Other CLI options
Some other CLI options are available in the CLI:
* `-d <device ID>`: You can provide a Webex Device ID to reuse previous registered Webex Devices you used before, if not provided, a new one is created.
* `-p <device ID>`: A Pixoo device ID, in case you have multiple devices and want to choose between them, otherwise an arbitrary device is selected.
# Disclaimers
* This crate does not have any official relations with Cisco.
* It's been tested only with Pixoo-64 devices at the moment.
* The GIFs used for the display must be 64x64 pixels GIFs.
