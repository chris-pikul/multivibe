fn main() {
    #[cfg(feature = "puck")]
    println!("Multivibe MK-I: Starting in Puck Mode (RPi)...");

    #[cfg(feature = "software")]
    println!("Multivibe MK-I: Starting in Software Mode (Desktop)...");
    
    println!("Strong foundations laid. Ready for the engine.");
}
