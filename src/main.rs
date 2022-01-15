#[macro_use] extern crate log;

use std::error::Error;
use std::time::Duration;
use tokio::time;

use structopt::StructOpt;

use btleplug::api::{bleuuid::BleUuid, Central, Peripheral, CentralEvent, Manager as _, ScanFilter};
use btleplug::platform::{Adapter, Manager};
use futures::stream::StreamExt;

use log::LevelFilter;

#[derive(Debug, StructOpt)]
#[structopt(name = "Rusty Bicycle", about = "todo")]
struct Opt {

    #[structopt(short, long, possible_values = &["off", "error", "warn", "info", "debug", "trace"], default_value = "error")]
    log_level: LevelFilter,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    Scan,
}

async fn get_central() -> Result<Adapter, Box<dyn Error>> {
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await.unwrap();

    let central = adapters.into_iter().nth(0).unwrap();

    return Ok(central);
}

async fn list_devices() -> Result<(), Box<dyn Error>> {
    let central = get_central().await?;
    let mut events = central.events().await?;

    central.start_scan(ScanFilter::default()).await?;

    println!("Peripherals:");

    while let Some(event) = events.next().await {
        match event {
            CentralEvent::DeviceDiscovered(id) => {
                let peripherals = central.peripherals().await?;

                let peripheral = peripherals.iter().find(| &p | p.id() == id )
                                        .expect("Peripheral must exist.");


                let properties = peripheral.properties().await?.unwrap();
                let is_connected = peripheral.is_connected().await?;
                let address = properties.address;
                let local_name = properties.local_name.unwrap_or(String::from("(peripheral name unknown)"));

                if is_connected {
                  print!("* ");
                } else {
                  print!("  ");
                }

                print!(
                    "{:?} : {:?}",
                    address,
                    local_name
                );
                println!("");
            },
            _ => {},

        }
    }

    return Ok(());
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let opt = Opt::from_args();

    pretty_env_logger::formatted_builder()
        .filter_level(opt.log_level)
        .init();

    debug!("{:?}", opt);


    // time::sleep(Duration::from_secs(10)).await;
    
    match opt.cmd {
        Command::Scan => list_devices().await?,
    }

    return Ok(());
}
