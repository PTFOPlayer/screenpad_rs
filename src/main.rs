#[macro_use]
extern crate lazy_static;

use std::{env, process::{Command}, thread::sleep, time::Duration};
mod installer;

lazy_static! {
    static ref ARGS:Vec<String> = {
        let mut args: Vec<String> = env::args().collect();
        args.append(&mut vec!["".to_owned()]);
        args
    };
    static ref ARG_1:String = ARGS[1].clone();
    static ref ARG_2:String = ARGS[2].clone();

    // change this to true if you want to use build in installer
    static ref INSTALLER_STATUS: bool = false;
}

fn main() {
    arg_parser();
}

fn arg_parser() {
    match ARG_1.as_str() {
        "--install" | "-i" => {
            if *INSTALLER_STATUS {
                if installer::install_asus_wmi() {println!("After reboot run:\n sudo chmod a+w '/sys/class/leds/asus::screenpad/brightness'", )} 
                else {println!("Error installing")}
            } else {println!("installer disabled, check github page for more info")}
        }
        "--brightness" | "-b" => {
            let brightness= ARG_2.as_str();
            let b_int:i32 = brightness.parse().unwrap();
            if b_int > 0 && b_int <= 100 {
                let mut set = Command::new("sh");
                let out = set
                .arg("/usr/bin/brightness.sh")
                .arg(brightness);
                match out.output() {
                    Ok(_)=>{println!("succesfully changed brightness")}
                    Err(err)=>{println!("error: {}",err)}
                }
            } else {println!("wrong value, correct values are in range of 1..100")}
        }
        "--overload" | "-o" => {
            let brightness= ARG_2.as_str();
            if brightness > "0" && brightness <= "255" {
                let mut set = Command::new("sh");
                let out = set
                .arg("/usr/bin/brightness.sh")
                .arg(brightness);
                match out.output() {
                    Ok(_)=>{println!("succesfully changed brightness")}
                    Err(err)=>{println!("error: {}",err)}
                }
            } else {println!("wrong value, correct values are in range of 1..255")}
        }
        "--off" | "-f" => {
            let mut set = Command::new("sh");
                let out = set
                .arg("/usr/bin/brightness.sh")
                .arg("0");
            match out.output() {
                Ok(_)=>{println!("succesfully changed brightness")}
                Err(err)=>{println!("error: {}",err)}
            }
        }
        "--on" | "-n" => {
            let mut set = Command::new("sh");
                let out = set
                .arg("/usr/bin/brightness.sh")
                .arg("100");
            match out.output() {
                Ok(_)=>{println!("succesfully changed brightness")}
                Err(err)=>{println!("error: {}",err)}
            }
        }
        "--help" | "-h" => {
            println!("
--brightness | -b : changes brightness in values from 1 to 100,
--overload | -o : changes brightness in extended range from 1 to 255,
--off | -f : turns screenpad off,
--on | -n : turns screenpad on")
        }
        &_ => {println!("wrong argument, see --help");}
    }
}

