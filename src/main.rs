use std::env::args;
use std::process::Command;

fn main() {
    let mut arguments = args().skip(1);

    let ip = arguments
        .next()
        .expect("Please provide an ip address to change to");

    let optional_subnet_mask = arguments.next();
    let mut subnet_mask = "255.255.0.0".to_owned();

    if optional_subnet_mask.is_some() {
        subnet_mask = optional_subnet_mask.unwrap();
    }

    let interface_name = "Ethernet";

    println!(
        "netsh interface ip set address name={} static {} {}",
        interface_name, ip, subnet_mask
    );

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

    Command::new("netsh")
        .arg("interface")
        .arg("set")
        .arg("interface")
        .arg(interface_name)
        .arg("admin=disable")
        .output()
        .expect("Failed to execute command to disable ethernet driver");

    println!("please wait for me to finish up t- 5s");

    let sleep_duration = std::time::Duration::from_millis(5000);
    std::thread::sleep(sleep_duration);

    println!(
        "netsh interface set interface {} admin=enable",
        interface_name
    );

    Command::new("netsh")
        .arg("interface")
        .arg("set")
        .arg("interface")
        .arg(interface_name)
        .arg("admin=enable")
        .output()
        .expect("Failed to execute command to enable ethernet driver");
}
