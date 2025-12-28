// Filename: src/lib.rs
// V 1.3 2024-12-28 Added get_state function for status endpoint
// V 1.2 2024-12-28 Removed J prefix from internal naming
// V 1.1 2024-12-28 Relay numbering changed from 2,3,4,5 to 1,2,3,4
// V 1.0 Initial version

use sysfs_gpio::{Direction, Pin};
use std::thread::sleep;
use std::time::Duration;

const RELAY_1: u64 = 60;
const RELAY_2: u64 = 27;
const RELAY_3: u64 = 85;
const RELAY_4: u64 = 86;

//*********************************
//  RelayBoard structure
//*********************************
pub struct RelayBoard {
    relay_1: Pin,
    relay_2: Pin,
    relay_3: Pin,
    relay_4: Pin,
}

//*********************************
//  RelayError enum
//*********************************
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
    //*********************************
    //  Initialize relay board
    //*********************************
    pub fn new() -> Result<Self, RelayError> {
        let relay_1 = Pin::new(RELAY_1);
        let relay_2 = Pin::new(RELAY_2);
        let relay_3 = Pin::new(RELAY_3);
        let relay_4 = Pin::new(RELAY_4);

        relay_1.export().map_err(|e| RelayError::GpioExport(e.to_string()))?;
        relay_2.export().map_err(|e| RelayError::GpioExport(e.to_string()))?;
        relay_3.export().map_err(|e| RelayError::GpioExport(e.to_string()))?;
        relay_4.export().map_err(|e| RelayError::GpioExport(e.to_string()))?;

        relay_1.set_direction(Direction::Out).map_err(|e| RelayError::GpioDirection(e.to_string()))?;
        relay_2.set_direction(Direction::Out).map_err(|e| RelayError::GpioDirection(e.to_string()))?;
        relay_3.set_direction(Direction::Out).map_err(|e| RelayError::GpioDirection(e.to_string()))?;
        relay_4.set_direction(Direction::Out).map_err(|e| RelayError::GpioDirection(e.to_string()))?;

        Ok(RelayBoard { relay_1, relay_2, relay_3, relay_4 })
    }

    //*********************************
    //  Turn relay ON
    //*********************************
    pub fn relay_on(&self, relay: u8) -> Result<(), RelayError> {
        match relay {
            1 => self.relay_1.set_value(1).map_err(|e| RelayError::GpioValue(e.to_string())),
            2 => self.relay_2.set_value(1).map_err(|e| RelayError::GpioValue(e.to_string())),
            3 => self.relay_3.set_value(1).map_err(|e| RelayError::GpioValue(e.to_string())),
            4 => self.relay_4.set_value(1).map_err(|e| RelayError::GpioValue(e.to_string())),
            _ => Err(RelayError::GpioValue(format!("Invalid relay number: {}", relay))),
        }
    }

    //*********************************
    //  Turn relay OFF
    //*********************************
    pub fn relay_off(&self, relay: u8) -> Result<(), RelayError> {
        match relay {
            1 => self.relay_1.set_value(0).map_err(|e| RelayError::GpioValue(e.to_string())),
            2 => self.relay_2.set_value(0).map_err(|e| RelayError::GpioValue(e.to_string())),
            3 => self.relay_3.set_value(0).map_err(|e| RelayError::GpioValue(e.to_string())),
            4 => self.relay_4.set_value(0).map_err(|e| RelayError::GpioValue(e.to_string())),
            _ => Err(RelayError::GpioValue(format!("Invalid relay number: {}", relay))),
        }
    }

    //*********************************
    //  Get relay state (0=off, 1=on)
    //*********************************
    pub fn get_state(&self, relay: u8) -> Result<u8, RelayError> {
        match relay {
            1 => self.relay_1.get_value().map_err(|e| RelayError::GpioValue(e.to_string())),
            2 => self.relay_2.get_value().map_err(|e| RelayError::GpioValue(e.to_string())),
            3 => self.relay_3.get_value().map_err(|e| RelayError::GpioValue(e.to_string())),
            4 => self.relay_4.get_value().map_err(|e| RelayError::GpioValue(e.to_string())),
            _ => Err(RelayError::GpioValue(format!("Invalid relay number: {}", relay))),
        }
    }

    //*********************************
    //  Turn all relays OFF
    //*********************************
    pub fn all_off(&self) -> Result<(), RelayError> {
        self.relay_off(1)?;
        self.relay_off(2)?;
        self.relay_off(3)?;
        self.relay_off(4)?;
        Ok(())
    }

    //*********************************
    //  Test sequence for all relays
    //*********************************
    pub fn test_sequence(&self) -> Result<(), RelayError> {
        println!("Testing all relays in sequence...");
        
        for relay in [1, 2, 3, 4] {
            println!("Activating relay {}", relay);
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
        let _ = self.relay_1.unexport();
        let _ = self.relay_2.unexport();
        let _ = self.relay_3.unexport();
        let _ = self.relay_4.unexport();
    }
}

// EOF
