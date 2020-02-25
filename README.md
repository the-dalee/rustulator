# rustulator

## Pinout

### Display

| Controller Pins | Display         |
|-----------------|-----------------|
| PA0-PA7         | data bits D0-D7 |
| PE0             | Enable          |
| PE1             | RW              |
| PE2             | Register select |

## Links

- [LCD display data sheet](https://www.sparkfun.com/datasheets/LCD/ADM1602K-NSW-FBS-3.3v.pdf)
- [Keypad data sheet](https://www.parallax.com/sites/default/files/downloads/27899-4x4-Matrix-Membrane-Keypad-v1.2.pdf)

## Running
```
cd /tmp && openocd -f interface/stlink-v2-1.cfg -f target/stm32f3x.cfg
cd /tmp && itmdump -F -f itm.txt
arm-none-eabi-gdb -x openocd.gdb -q target/thumbv7em-none-eabihf/debug/rustulator
```