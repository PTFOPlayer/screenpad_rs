cargo build --release
sudo cp target/release/screenpad_rs /usr/bin
sudo cp ./src/brightness.sh /usr/bin/brightness.sh
sudo cp ./src/current.sh /usr/bin/current.sh
sudo cp ./screenpad_rs.service /etc/systemd/system/
sudo mkdir /var/screenpad_rs/
sudo echo 100 > /var/screenpad_rs/brightness