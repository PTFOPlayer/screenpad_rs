# screenpad_rs
Simple CLI wrapper to change brightness of asus screenpad


# instalation
  ### dependencies:
    * rustup
    * wget
    * dkms
    * linux-headers
  ### automatic install
    * install [Plippo asus-wmi-screenpad repo](https://github.com/Plippo/asus-wmi-screenpad)
    * clone repository
    ```
    git clone https://github.com/PTFOPlayer/screenpad_rs && cd screenpad_rs
    ```
    * run
    ```
    ./install.sh
    ```
  ### steps:
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
    sudo mkdir /usr/share/screenpad_rs
    sudo echo 100 > /usr/share/screenpad_rs/brightness
    ```
  ### automatic synchronization
    * if you want to use synchronization service you need to do:
    ```
    sudo systemctl start screenpad_rs.service
    sudo systemctl enable screenpad_rs.service
    ```
# usage
| flag | simplified | what is doing | 
| ---| --- | --- |
| --brightness | -b |   : changes brightness in values from 1 to 255, |
| --sync | -s |   : synchronizes brightness of screenpad with main screen |
| --current | -c |   : gives current brightness of screenpad |
| --off | -f |   : turns screenpad off, |
| --on | -n |   : turns screenpad on, |
| --up | -u |   : increases brightness by 10 |
| --down | -d |   : decreases brightness by 10 |
| --watch | -w |   : automatic sync |

# TODO
  * touch mapping on/off
  * brightness profiles
  * GUI 

# DONE
  * install.sh
  * brightenss info 
  * synchronization service