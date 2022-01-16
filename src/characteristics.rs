use int_enum::IntEnum;
use btleplug::api::bleuuid;
use btleplug::api::bleuuid::BleUuid;

use uuid::Uuid;



#[repr(u16)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum)]
pub enum FitnessCharacteristic {
    ServiceChanged = 0x2A05,

    SerialNumberString     = 0x2A25,
    FirmwareRevisionString = 0x2A26,
    HardwareRevisionString = 0x2A27,
    SoftwareRevisionString = 0x2A28,
    ManufacturerNameString = 0x2A29,

    SensorLocation = 0x2A5D,

    CSCMeasurement = 0x2A5B,
    CSCFeature     = 0x2A5C,

    CyclingPowerMeasurement  = 0x2A63,
    CylcingPowerFeature      = 0x2A65,
    CyclingPowerControlPoint = 0x2A66,

    FitnessMachineFeature         = 0x2ACC,
    IndoorBikeData                = 0x2AD2,
    TrainingStatus                = 0x2AD3,
    SupportedResistanceLevelRange = 0x2AD6,
    SupportedPowerRange           = 0x2AD8,
    FitnessMachineControlPoint    = 0x2AD9,
    FitnessMachineStatus          = 0x2ADA,


}

impl FitnessCharacteristic {
    pub fn name(&self) -> &str {
        match self {
            FitnessCharacteristic::ServiceChanged => "Service Changed",

            FitnessCharacteristic::SerialNumberString     => "Serial Number String",
            FitnessCharacteristic::FirmwareRevisionString => "Firmware Revision String",
            FitnessCharacteristic::HardwareRevisionString => "Harware Revision String",
            FitnessCharacteristic::SoftwareRevisionString => "Software Revision String",
            FitnessCharacteristic::ManufacturerNameString => "Manufacturer Name String",

            FitnessCharacteristic::SensorLocation => "Sensor Location",

            FitnessCharacteristic::CSCMeasurement => "CSC Measurement",
            FitnessCharacteristic::CSCFeature     => "CSC Feature",

            FitnessCharacteristic::CyclingPowerMeasurement  => "Cycling Power Measurement",
            FitnessCharacteristic::CylcingPowerFeature      => "Cycling Power Feature",
            FitnessCharacteristic::CyclingPowerControlPoint => "Cycling Power Control Point",

            FitnessCharacteristic::FitnessMachineFeature         => "Fitness Machine Feature",
            FitnessCharacteristic::IndoorBikeData                => "Indoor Bike Data",
            FitnessCharacteristic::TrainingStatus                => "Training Status",
            FitnessCharacteristic::SupportedResistanceLevelRange => "Supported Resistance Level Range",
            FitnessCharacteristic::SupportedPowerRange           => "Supported Power Range",
            FitnessCharacteristic::FitnessMachineControlPoint    => "Fitness Machine Control Point",
            FitnessCharacteristic::FitnessMachineStatus          => "Fittness Machine Status",
        }
    }

    pub fn uuid(self) -> Uuid {
        return bleuuid::uuid_from_u16(self as u16);
    }

    pub fn from_uuid(uuid: Uuid) -> Result<FitnessCharacteristic, &'static str> {
        match uuid.to_ble_u16() {
            Some(id) => {
                match FitnessCharacteristic::from_int(id) {
                    Ok(service) => Ok(service),
                    _ => Err("No matching Service"),
                }
            },
            _ => return Err("uuid is not a valid BleUuid"),
        }
    }
}
