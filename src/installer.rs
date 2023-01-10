use crate::*;
lazy_static! {
    #[derive(Debug)]
    static ref DEPENDENCIES: String = "dkms, wget, linux-headers".to_owned();
    static ref COMMANDS: Vec<String>= {vec![
    "sudo mkdir /usr/src/asus-wmi-1.0".to_owned(),
    "sudo wget -P /usr/src/asus-wmi-1.0 https://github.com/Plippo/asus-wmi-screenpad/archive/master.zip".to_owned(),
    "sudo unzip /usr/src/asus-wmi-1.0/master.zip -d /usr/src/asus-wmi-1.0".to_owned(),
    "sudo mv /usr/src/asus-wmi-1.0/asus-wmi-screenpad-master/* /usr/src/asus-wmi-1.0/".to_owned(),
    "sudo rm -rf /usr/src/asus-wmi-1.0/master.zip".to_owned(),
    "sudo sh /usr/src/asus-wmi-1.0/prepare-for-current-kernel.sh".to_owned()
    ]};   
}

pub fn install_asus_wmi() -> bool{
    println!("instalation of asus_wmi is fully automatic, before begining please install packages: {:?}\n
    INSTALLER IS NOT FULLY DEVELOPED YET, PLEASE CONSIDER INSTALLING PACKAGES YOURSELF", DEPENDENCIES.as_str());
    println!("instalation will begin in 5 seconds");
    sleep(Duration::from_secs(5));
    let commands: Vec<String> = COMMANDS.to_vec();
    for command in commands {
        let splitted:Vec<&str> = command.split_whitespace().collect();
        let program = splitted[0].clone(); 
        let mut arguments: Vec<String> = vec![];
        
        for arg in splitted.clone() {
            if arg.ne(program) {arguments.append(&mut vec![arg.to_owned()]);}
        }
        
        let mut executor = Command::new(program);
        executor.args(arguments.clone());
        let output = executor.output().expect(&format!("error ocured executing command {:?}", command));
        
        println!("{}{}", String::from_utf8(output.stdout).unwrap(), String::from_utf8(output.stderr).unwrap());

        if !output.status.success(){
            return false
        }
    }
    true
}