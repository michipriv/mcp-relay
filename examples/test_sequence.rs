use relay_controller::{RelayBoard, RelayError};

fn main() -> Result<(), RelayError> {
    println!("Rock Pi E - Relay Board Controller");
    println!("===================================");
    
    let board = RelayBoard::new()?;
    println!("RelayBoard initialized successfully");
    
    board.test_sequence()?;
    
    Ok(())
}
