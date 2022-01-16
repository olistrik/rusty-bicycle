

use int_enum::IntEnum;
use btleplug::api::bleuuid;
use btleplug::api::bleuuid::BleUuid;

use uuid::Uuid;



#[repr(u16)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
pub enum FitnessService {
    GenericAttribute   =  0x1801,
    DeviceInformation   = 0x180A,
    FitnessMachine      = 0x1826,
    RunningSpeedCadence = 0x1814,
    CyclingSpeedCadence = 0x1816,
    CyclingPower        = 0x1818,
}

impl FitnessService {
    pub fn name(&self) -> &str {
        match self {
            FitnessService::GenericAttribute   => return "Generic Attribute",
            FitnessService::DeviceInformation   => return "Device Information",
            FitnessService::FitnessMachine      => return "Fitness Machine",
            FitnessService::RunningSpeedCadence => return "Running Speed and Cadence",
            FitnessService::CyclingSpeedCadence => return "Cyling Speed and Cadence",
            FitnessService::CyclingPower        => return "Cyling Power",
        }
    }

    pub fn uuid(self) -> Uuid {
        return bleuuid::uuid_from_u16(self as u16);
    }

    pub fn from_uuid(uuid: Uuid) -> Result<FitnessService, &'static str> {
        match uuid.to_ble_u16() {
            Some(id) => {
                match FitnessService::from_int(id) {
                    Ok(service) => Ok(service),
                    _ => Err("No matching Service"),
                }
            },
            _ => return Err("uuid is not a valid BleUuid"),
        }
    }
}
