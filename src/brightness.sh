echo "$@" | sudo tee '/sys/class/leds/asus::screenpad/brightness'
echo "$@" > /usr/share/screenpad_rs/brightness