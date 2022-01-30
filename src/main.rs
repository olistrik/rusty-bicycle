pub mod services;
pub mod characteristics;

#[macro_use]
extern crate log;

use std::error::Error;

use structopt::StructOpt;

use btleplug::api::{Central, CentralEvent, Manager as _, Peripheral, BDAddr, ScanFilter};
use btleplug::platform::{Adapter, Manager};
use futures::stream::StreamExt;

use log::LevelFilter;

use uuid::Uuid;
use services::FitnessService;
use characteristics::FitnessCharacteristic;

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
    #[structopt(name = "scan")]
    /// Scan for fitness machines.
    Scan {

        #[structopt(long)]
        /// Don't filter non-fitness machines from the scan results.
        no_filter: bool,

        #[structopt(long)]
        /// Filter devices by the given service uuid.
        filter: Option<Uuid>,
    },

    Debug {
        #[structopt(
            required = true,
        )]
        device: BDAddr,

        #[structopt(subcommand)]
        cmd: Debug,
    }
}

#[derive(Debug, StructOpt)]
enum Debug {
    #[structopt(name = "services")]
    Services {  }
}

async fn get_central() -> Result<Adapter, Box<dyn Error>> {
    let manager = Manager::new().await?;
    let adapters = manager.adapters().await.unwrap();

    let central = adapters.into_iter().nth(0).unwrap();

    return Ok(central);
}

async fn list_devices(no_filter: bool, filter: Option<Uuid>) -> Result<(), Box<dyn Error>> {
    let central = get_central().await?;
    let mut events = central.events().await?;

    info!("Starting scan on {}...", central.adapter_info().await?);

    let scan_filter;

    let unwrapped_filter = filter.unwrap_or(FitnessService::FitnessMachine.uuid());

    if no_filter {
        scan_filter = ScanFilter::default();
    } else {
        println!("filter: {}", unwrapped_filter);
        scan_filter = ScanFilter { services: vec![unwrapped_filter] };
    }

    central.start_scan(scan_filter).await?;

    println!("BLE Fitness Machines:");

    while let Some(event) = events.next().await {
        match event {
            CentralEvent::DeviceDiscovered(id) => {
                let peripheral = central.peripheral(&id).await?;

                let properties = peripheral.properties().await?.unwrap();
                let is_connected = peripheral.is_connected().await?;
                let address = properties.address;
                let local_name = properties
                    .local_name
                    .unwrap_or(String::from("(peripheral name unknown)"));

                if is_connected {
                    print!("* ");
                } else {
                    print!("  ");
                }

                print!("{:?} : {:?}", address, local_name);
                println!("");
            }
            _ => {}
        }
    }

    return Ok(());
}


async fn debug_services(device: BDAddr) -> Result<(), Box<dyn Error>> {
    let central = get_central().await?;
    let mut events = central.events().await?;

    debug!("Searching for {}...", device);

    central.start_scan(ScanFilter::default()).await?;
    
    while let Some(event) = events.next().await {
        match event {
            CentralEvent::DeviceDiscovered(id) => {
                let peripheral = central.peripheral(&id).await?;

                if peripheral.address() == device {
                    debug!("Connecting to {}...", device);
                    peripheral.connect().await?;

                    debug!("Discovering Services...");
                    peripheral.discover_services().await?;

                    for service in peripheral.services() {
                        println!("");
                        match FitnessService::from_uuid(service.uuid) {
                            Ok(s) => println!("{} Service", s.name()),
                            _ => println!("Unkown Service: {}", service.uuid)
                        }

                        for characteristic in service.characteristics {
                            match FitnessCharacteristic::from_uuid(characteristic.uuid) {
                                Ok(c) => println!("  - {}", c.name()),
                                _ => println!("  - Unknown Characteristic: {}", characteristic.uuid),
                            }
                            println!("    [{:?}]", characteristic.properties);
                        }
                    }

                    debug!("Disconnecting from {}...", device);
                    peripheral.disconnect().await?;

                    central.stop_scan().await?;

                    return Ok(());
                }
            }
            _ => {}
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
        Command::Scan { no_filter, filter }  => list_devices(no_filter, filter).await?,
        Command::Debug { device, cmd } => {
            match cmd {
                Debug::Services {} => debug_services(device).await?,
            }
        }
    }

    return Ok(());
}
