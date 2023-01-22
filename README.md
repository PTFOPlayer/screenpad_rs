# screenpad_rs
Simple CLI wrapper to change brightness of asus screenpad


# instalation
  ### dependencies:
    * rustup
    * wget
    * dkms
    * linux-headers
  ### things to do:
  * install [Plippo asus-wmi-screenpad repo](https://github.com/Plippo/asus-wmi-screenpad)
  * clone repository
  ```
  git clone https://github.com/PTFOPlayer/screenpad_rs && cd screenpad_rs
  ```
  * now you can run build
  ```
  cargo build --release
  ```
  * lastly just coppy needed files and create place for config
  ```
  sudo cp target/release/screenpad_rs /usr/bin
  sudo cp ./src/brightness.sh /usr/bin/brightness.sh
  sudo cp ./src/current.sh /usr/bin/current.sh
  mkdir ~/.config/screenpad_rs
  ```

  * if you want to use synchronization service you need to do:
  ```
  sudo cp ./screenpad_rs.service /etc/systemd/system/
  sudo systemctl start screenpad_rs.service
  ```

# TODO
  * touch mapping on/off
  * brightness profiles
  * install.sh 
  * GUI 

# DONE
  * brightenss info 
  * synchronization service