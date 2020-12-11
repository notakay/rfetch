use nixinfo::{cpu, distro, gpu, hostname, kernel, uptime};
use std::env;

fn main() {

    if let Some(username) = get_username() {
        if let Some(hostname) = get_hostname() {
            println!("\x1b[1m{}@{}\x1b[0m", username, hostname);
            println!("\x1b[1m---\x1b[0m")
        }
    }

    if let Some(i) = get_distro() {
        print_pretty("distro: ", &i[1..i.len()-1]);
    }

    if let Some(i) = get_hostname() {
        print_pretty("hostname: ", &i);
    }

    if let Some(i) = get_kernel() {
        print_pretty("kernel: ", &i);
    }

    if let Some(i) = get_uptime() {
        print_pretty("uptime: ", &i);
    }

    if let Some(i) = get_cpu() {
        print_pretty("CPU: ", &i);
    }

    if let Some(i) = get_gpu() {
        print_pretty("GPU: ", &i);
    }
}

fn print_pretty(s1: &str, s2: &str) {
    print!("\x1b[1m{}\x1b[0m", s1);
    println!("{}", s2);
}

fn get_cpu() -> Option<String> {
    match cpu() {
        Ok(i) => Some(i),
        Err(_) => None
    }
}

fn get_distro() -> Option<String> {
    match distro() {
        Ok(i) => Some(i),
        Err(_) => None
    }
}

fn get_gpu() -> Option<String> {
    match gpu() {
        Ok(i) => Some(i),
        Err(_) => None
    }
}

fn get_hostname() -> Option<String> {
    match hostname() {
        Ok(i) => Some(i),
        Err(_) => None
    }
}


fn get_kernel() -> Option<String> {
    match kernel() {
        Ok(i) => Some(i),
        Err(_) => None
    }
}

fn get_uptime() -> Option<String> {
    match uptime() {
        Ok(i) => Some(i),
        Err(_) => None
    }
}

fn get_username() -> Option<String> {
    match env::var("USER") {
        Ok(i) => Some(i),
        Err(_) => None,
    }
}