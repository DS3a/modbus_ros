use modbus::tcp;
use modbus::Client;
use std::{thread, time};

fn main() {
    let config = tcp::Config::default();
    println!("Attempting to establish connection with the motor");
    let mut client = tcp::Transport::new_with_cfg("10.42.0.42", config).unwrap();
    println!("Motor connection established");

    /*    let mut data = client.read_input_registers(0x2B, 0x2B).unwrap();
    dbg!(data);*/

    println!("Attempting to read registers");
    let mut register: u16 = 0x00 as u16;
    loop {
        println!("Reading the register : {:#02x}", register);
        let data = client.write_single_register(register as u16, 12 as u16);
        println!("Here is the output : ");
        dbg!(&data);
        register += 1;
        thread::sleep(time::Duration::from_millis(300));
    }

    //    println!("Data has been read, here is the output : ");
    //    dbg!(&_data);
}
