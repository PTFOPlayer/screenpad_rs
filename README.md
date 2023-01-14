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
  * lastly just coppy needed files and every thing is done
  ```
  sudo cp target/release/screenpad_rs /usr/bin
  sudo cp /src/brightness.sh /usr/bin/brightness.sh
  ```
# TODO
  * touch mapping on/off
  * brightenss info 
  * GUI 
