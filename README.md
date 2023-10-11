# Screenpad_rs
Simple CLI wrapper to change brightness of asus screenpad


# Install
### Pre-compiled binaries
Precompiled binaries are in screenpad_rs_binaries.tar.gz

### Dependencies:
* rustup
* wget
* dkms
* linux-headers

Install [Plippo asus-wmi-screenpad repo](https://github.com/Plippo/asus-wmi-screenpad), clone repository
```
git clone https://github.com/PTFOPlayer/screenpad_rs && cd screenpad_rs
```
### Automatic install
```
./install.sh
```
### Steps

1. Run build
```
cargo build --release
```
2. Copy needed files and create place for config
```
sudo cp target/release/screenpad_rs /usr/bin
sudo cp ./src/brightness.sh /usr/bin/brightness.sh
sudo cp ./src/current.sh /usr/bin/current.sh
sudo mkdir /usr/share/screenpad_rs
sudo echo 100 > /usr/share/screenpad_rs/brightness
```
### Automatic synchronization
To use synchronization service:
```
sudo systemctl start screenpad_rs.service
sudo systemctl enable screenpad_rs.service
```
# Usage
| flag         | simplified | what is doing                                         |
| ------------ | ---------- | ----------------------------------------------------- |
| --brightness | -b         | changes brightness in values from 1 to 255,           |
| --sync       | -s         | synchronizes brightness of screenpad with main screen |
| --current    | -c         | gives current brightness of screenpad                 |
| --off        | -f         | turns screenpad off,                                  |
| --on         | -n         | turns screenpad on,                                   |
| --up         | -u         | increases brightness by 10                            |
| --down       | -d         | decreases brightness by 10                            |
| --watch      | -w         | automatic sync                                        |

### TODO
* Touch mapping on/off
* Brightness profiles
* GUI 
