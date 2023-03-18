echo "$@" | sudo tee '/sys/class/leds/asus::screenpad/brightness'
echo "$@" > /var/screenpad_rs/brightness