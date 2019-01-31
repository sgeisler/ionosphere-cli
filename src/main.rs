extern crate ionosphere;
extern crate shellexpand;
extern crate structopt;

use std::borrow::Borrow;
use std::process::exit;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "ionosphere", about = "A cli tool to broadcast files using blockstream satellite.")]
struct CliOptions {
    #[structopt(
        short = "l",
        default_value = "~/.lightning/lightning-rpc",
        help = "path to the lightningd rpc socket"
    )]
    pub linghtningd: String,
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "fundchannel", help = "fund a direct channel to the API's lightning node")]
    FundChannel{
        amount_sat: u32,
    },
    #[structopt(name = "broadcast", help = "bid for broadcasting a file")]
    Bid {
        file: String,
        amount_msat: u64,
    },
}

fn main() {
    let options: CliOptions = CliOptions::from_args();

    let extended_lightningd_path = shellexpand::tilde(&options.linghtningd);
    let mut api = ionosphere::IonosphereClient::new_blockstream_client(
        Borrow::<str>::borrow(&extended_lightningd_path)
    );

    match options.command {
        Command::FundChannel {amount_sat} => {
            api.open_channel(amount_sat).unwrap_or_else(|e| {
                eprintln!("An error occurred while opening the channel: {:?}", e);
                exit(-1);
            });
        },
        Command::Bid {file, amount_msat} => {
            let expanded_path = shellexpand::tilde(&file);
            api.place_bid(
                Borrow::<str>::borrow(&expanded_path),
                amount_msat
            ).unwrap_or_else(|e| {
                eprintln!("An error occurred while placing the bid: {:?}", e);
                exit(-2);
            });
        },
    }
}
