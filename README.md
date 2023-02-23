# Serial Logger
This project logs a serial port and writes the content in a file on the system.

The first parameter is the serial port and the seconde the baudrate.

## Example
```text
./serial_logger /dev/ttyUSB0 115200
```

This creates a folder in the home directory with "log_files/<YEAR>/<YEAR-MONTH>/<YEAR-MONTH-DAY>/<HHMM>.log".

Additionally, the logger writes the output on the screen, where it is called. So it loggs and shows the serial output of e.g. a device.
