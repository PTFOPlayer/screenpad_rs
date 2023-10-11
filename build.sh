cargo build --release
cp ./target/release/screenpad_rs ./build/screenpad_rs_binaries/
cp ./screenpad_rs.service ./build/screenpad_rs_binaries/
tar -zcvf screenpad_rs_binaries.tar.gz ./build/screenpad_rs_binaries/ 