fn main() {
    const STARTING_MISSILES: i32 = 8;
    const READY_AMOUNT: i32 = 2;

    let (missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);

    println!("Firing {} out of {}!", ready, missiles - ready);
}
