st-info --descr
st-flash erase
erase oled_display.bin
cargo build --release
cargo objcopy --bin oled_display --target thumbv7m-none-eabi --release -- -O binary oled_display.bin
st-flash write oled_display.bin 0x8000000