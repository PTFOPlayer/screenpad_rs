#[macro_use]
extern crate lazy_static;

use std::{
    env,
    process::{exit, Command},
    thread::sleep,
    time::Duration,
};

use sudo::{check, escalate_if_needed};

lazy_static! {
    static ref ARGS: Vec<String> = {
        let mut args: Vec<String> = env::args().collect();
        args.append(&mut vec!["".to_owned()]);
        args
    };
    static ref ARG_1: String = ARGS[1].clone();
    static ref ARG_2: String = ARGS[2].clone();
}

fn main() {
    arg_parser();
}

fn arg_parser() {
    match ARG_1.as_str() {
        "--brightness" | "-b" => brightness(),
        "--off" | "-f" => off(),
        "--on" | "-n" => on(),
        "--sync" | "-s" => sync(),
        "--current" | "-c" => println!("current screenpad brightness: {}", current()),
        "--up" | "-u" => up(),
        "--down" | "-d" => down(),
        "--watch" | "-w" => watch(),
        "--help" | "-h" => help(),
        &_ => {
            println!("wrong argument, see --help");
        }
    }
}

#[inline(always)]
fn execute(brightness: &str) {
    std::fs::write(
        "/sys/class/leds/asus::screenpad/brightness",
        brightness.as_bytes(),
    )
    .expect("error changing brigtness");
}

fn current() -> String {
    let file = std::fs::read("/sys/class/leds/asus::screenpad/brightness");
    match file {
        Ok(res) => String::from_utf8(res[..(res.len() - 1)].to_vec()).unwrap(),
        Err(_) => {
            println!("error ocured reading brightness");
            exit(1)
        }
    }
}

fn brightness() {
    let brightness = ARG_2.as_str();
    let b_int: i32 = match brightness.parse() {
        Ok(res) => res,
        Err(_) => {
            println!("wrong value provided, setting value to 100");
            100
        }
    };
    if b_int > 0 && b_int <= 255 {
        execute(&format!("{}", b_int));
    } else {
        println!("wrong value, correct values are in range of 1..255")
    }
}

fn off() {
    let mut ctl = Command::new("sudo");
    let out = ctl
        .args(["systemctl", "stop", "screenpad_rs.service"])
        .output();
    match out {
        Ok(_) => {
            println!("screenpad off (brightness set to 0, turning off actually only works on KDE)");
            execute("0");
        }
        Err(_) => {
            println!("couldn't stop process `screenpad_rs.service`")
        }
    }
}

fn on() {
    let mut ctl = Command::new("sudo");
    let out = ctl
        .args(["systemctl", "start", "screenpad_rs.service"])
        .output();
    match out {
        Ok(_) => {
            println!("screenpad on ");
        }
        Err(_) => {
            println!("couldn't stop process `screenpad_rs.service`")
        }
    }
}

fn sync() {
    let file = std::fs::read("/sys/class/backlight/intel_backlight/brightness");
    match file {
        Ok(res) => {
            let val_str = String::from_utf8(res[..(res.len() - 1)].to_vec()).unwrap();
            let val = val_str
                .parse::<i32>()
                .expect("error ocured while getting main screen brightness");
            let val = val / 98;
            execute(&format!("{}", val));
        }
        Err(err) => {
            println!("error: {}", err)
        }
    }
}

fn watch() {
    match check() {
        sudo::RunningAs::Root => {}
        _ => {
            let _ = escalate_if_needed();
        }
    };
    let mut prev = 0;
    loop {
        let file = std::fs::read("/sys/class/backlight/intel_backlight/brightness")
            .expect("error reading brightness from main screen");

        let val_str = String::from_utf8(file[..(file.len() - 1)].to_vec()).unwrap();

        let mut val = val_str
            .parse::<i32>()
            .expect("error ocured while getting main screen brightness");
        val = val / 98 + 1;

        if val != prev {
            prev = val;
            execute(&format!("{}", val));
        }

        sleep(Duration::from_secs_f32(0.2))
    }
}



fn up() {
    match current().parse::<i32>() {
        Ok(res) => {
            if res + 10 < 255 {
                execute(&format!("{}", res + 10))
            } else {
                println!("cannot increase more")
            }
        }
        Err(_) => println!("error occured increasing brightness"),
    }
}

fn down() {
    match current().parse::<i32>() {
        Ok(res) => {
            if res - 10 > 0 {
                execute(&format!("{}", res - 10))
            } else {
                println!("cannot decrease more")
            }
        }
        Err(_) => println!("error occured decreasing brightness"),
    }
}

fn help() {
    println!(
        "
--brightness    | -b    : changes brightness in values from 1 to 255,
--sync          | -s    : synchronizes brightness of screenpad with main screen
--current       | -c    : gives current brightness of screenpad
--off           | -f    : turns screenpad off,
--on            | -n    : turns screenpad on,
--up            | -u    : increases brightness by 10
--down          | -d    : decreases brightness by 10
--watch         | -w    : automatic sync
"
    );
}
