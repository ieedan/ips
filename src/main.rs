use std::env::args;
use std::process::Command;

fn main() {
    let mut arguments = args().skip(1); // Gets arguments

    // Tries to get the ip address from the arguments
    let ip = arguments
        .next()
        .expect("Please provide an ip address to change to");

    let interface_name = "Ethernet";

    if ip == "clr" {
        Command::new("netsh")
        .arg("interface")
        .arg("ip")
        .arg("set")
        .arg("address")
        .arg("\"".to_owned() + interface_name + "\"")
        .arg("dhcp")
        .output()
        .expect("Failed to execute command to go back to DHCP");

        return;
    }

    // Checks for the subnet mask
    let optional_subnet_mask = arguments.next();
    let mut subnet_mask = "255.255.0.0".to_owned();

    // Since subnet mask is optional if it is not provided it will keep the default setting 
    if optional_subnet_mask.is_some() {
        subnet_mask = optional_subnet_mask.unwrap();
    }

    println!(
        "netsh interface ip set address name={} static {} {}",
        interface_name, ip, subnet_mask
    );

    // Executes command to change IP address and subnet mask
    Command::new("netsh")
        .arg("interface")
        .arg("ip")
        .arg("set")
        .arg("address")
        .arg("name=".to_owned() + interface_name)
        .arg("static")
        .arg(ip)
        .arg(subnet_mask)
        .output()
        .expect("Failed to execute command to change ip");

    println!(
        "netsh interface set interface {} admin=disable",
        interface_name
    );

    // Executes command to disable interface
    Command::new("netsh")
        .arg("interface")
        .arg("set")
        .arg("interface")
        .arg(interface_name)
        .arg("admin=disable")
        .output()
        .expect("Failed to execute command to disable ethernet driver");

    println!("please wait for me to finish up t- 5s");

    // Waits 5 seconds so that there is not conflict when re-enabling
    let sleep_duration = std::time::Duration::from_millis(5000);
    std::thread::sleep(sleep_duration);

    println!(
        "netsh interface set interface {} admin=enable",
        interface_name
    );

    // Executes command to enable interface
    Command::new("netsh")
        .arg("interface")
        .arg("set")
        .arg("interface")
        .arg(interface_name)
        .arg("admin=enable")
        .output()
        .expect("Failed to execute command to enable ethernet driver");
}
