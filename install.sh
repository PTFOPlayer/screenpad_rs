mkdir build
cd build
tar -xvf ../screenpad_rs_binaries.tar.gz
cp screenpad_rs_binaries/* .

sudo cp screenpad_rs /usr/bin
sudo cp screenpad_rs.service /etc/systemd/system/
