echo "$@" | sudo tee '/sys/class/leds/asus::screenpad/brightness'
echo "$@" > ~/.config/screenpad_rs/brightness