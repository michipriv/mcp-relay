use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;

const RELAY_J2: u64 = 60;
const RELAY_J3: u64 = 27;
const RELAY_J4: u64 = 85;
const RELAY_J5: u64 = 86;

pub struct RelayBoard {
    j2: Pin,
    j3: Pin,
    j4: Pin,
    j5: Pin,
}

#[derive(Debug)]
pub enum RelayError {
    GpioExport(String),
    GpioDirection(String),
    GpioValue(String),
}

impl std::fmt::Display for RelayError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RelayError::GpioExport(e) => write!(f, "GPIO Export Error: {}", e),
            RelayError::GpioDirection(e) => write!(f, "GPIO Direction Error: {}", e),
            RelayError::GpioValue(e) => write!(f, "GPIO Value Error: {}", e),
        }
    }
}

impl std::error::Error for RelayError {}

impl RelayBoard {
    pub fn new() -> Result<Self, RelayError> {
        let j2 = Pin::new(RELAY_J2);
        let j3 = Pin::new(RELAY_J3);
        let j4 = Pin::new(RELAY_J4);
        let j5 = Pin::new(RELAY_J5);

        j2.export().map_err(|e| RelayError::GpioExport(e.to_string()))?;
        j3.export().map_err(|e| RelayError::GpioExport(e.to_string()))?;
        j4.export().map_err(|e| RelayError::GpioExport(e.to_string()))?;
        j5.export().map_err(|e| RelayError::GpioExport(e.to_string()))?;

        j2.set_direction(Direction::Out).map_err(|e| RelayError::GpioDirection(e.to_string()))?;
        j3.set_direction(Direction::Out).map_err(|e| RelayError::GpioDirection(e.to_string()))?;
        j4.set_direction(Direction::Out).map_err(|e| RelayError::GpioDirection(e.to_string()))?;
        j5.set_direction(Direction::Out).map_err(|e| RelayError::GpioDirection(e.to_string()))?;

        Ok(RelayBoard { j2, j3, j4, j5 })
    }

    pub fn relay_on(&self, relay: u8) -> Result<(), RelayError> {
        match relay {
            2 => self.j2.set_value(1).map_err(|e| RelayError::GpioValue(e.to_string())),
            3 => self.j3.set_value(1).map_err(|e| RelayError::GpioValue(e.to_string())),
            4 => self.j4.set_value(1).map_err(|e| RelayError::GpioValue(e.to_string())),
            5 => self.j5.set_value(1).map_err(|e| RelayError::GpioValue(e.to_string())),
            _ => Err(RelayError::GpioValue(format!("Invalid relay number: {}", relay))),
        }
    }

    pub fn relay_off(&self, relay: u8) -> Result<(), RelayError> {
        match relay {
            2 => self.j2.set_value(0).map_err(|e| RelayError::GpioValue(e.to_string())),
            3 => self.j3.set_value(0).map_err(|e| RelayError::GpioValue(e.to_string())),
            4 => self.j4.set_value(0).map_err(|e| RelayError::GpioValue(e.to_string())),
            5 => self.j5.set_value(0).map_err(|e| RelayError::GpioValue(e.to_string())),
            _ => Err(RelayError::GpioValue(format!("Invalid relay number: {}", relay))),
        }
    }

    pub fn all_off(&self) -> Result<(), RelayError> {
        self.relay_off(2)?;
        self.relay_off(3)?;
        self.relay_off(4)?;
        self.relay_off(5)?;
        Ok(())
    }

    pub fn test_sequence(&self) -> Result<(), RelayError> {
        println!("Testing all relays in sequence...");
        
        for relay in [2, 3, 4, 5] {
            println!("Activating relay J{}", relay);
            self.relay_on(relay)?;
            sleep(Duration::from_millis(1000));
            self.relay_off(relay)?;
            sleep(Duration::from_millis(500));
        }
        
        println!("Test sequence complete");
        Ok(())
    }
}

impl Drop for RelayBoard {
    fn drop(&mut self) {
        let _ = self.all_off();
        let _ = self.j2.unexport();
        let _ = self.j3.unexport();
        let _ = self.j4.unexport();
        let _ = self.j5.unexport();
    }
}
