# Avr window opener project

This crate can be used only for atmega128rfa1 or atmega328p mcu based boards

### Compile the crate to an ELF executable

`cargo build --target <target> --features <mcu> --release`

Available targets and appropriate features:

|         target         |    feature    |
| :--------------------: | :-----------: |
| avr-atmega128rfa1.json | atmega128rfa1 |
|  avr-atmega328p.json   |  atmega328p   |

### SIGNALS

Each request must end with ';' symbol

|    Signal type    |       Request       |                      Response                       |
| :---------------: | :-----------------: | :-------------------------------------------------: |
|    open window    |         'o'         |                        null                         |
|   close window    |         'c'         |                        null                         |
| get window state  |         's'         |                  'o' \| 'c' \*(1)                   |
|     get time      |         't'         |                 't' + 4 bytes \*(2)                 |
|    update time    | 'u' + 4 bytes \*(2) |        'set_time_ok' \| 'set_time_err' \*(3)        |
|   set time mode   | 'r' + 8 bytes \*(4) |  'enable_ok' + 'schedule_disabled' \| 'enable_err'  |
| enable time mode  |         'e'         |  'enable_ok' + 'schedule_disabled' \| 'enable_err'  |
| disable time mode |         'd'         |                    'disable_ok'                     |
|   get time mode   |         'a'         |                    8 bytes \*(4)                    |
|   set schedule    | 'h' + 8 bytes \*(5) | 'schedule_enabled' + 'disable_ok' \| 'schedule_err' |
|  enable schedule  |  'enable_schedule'  | 'schedule_enabled' + 'disable_ok' \| 'schedule_err' |
| disable schedule  | 'disable_schedule'  |                 'schedule_disabled'                 |
| get schedule time |         'b'         |                    8 bytes \*(5)                    |

\*(1) o - open, c - close  
\*(2) 4 bytes is u32 time value in seconds  
\*(3) set_time_err will be sended if time value >= 86400 (i.e. greater than 23:59:59)  
\*(4) the first 4 bytes - time in opened position, the second 4 bytes - time in closed position  
\*(5) the first 4 bytes - time to open, the second 4 bytes - time to close

#### Presentation avaialble at <https://nikbel3476.github.io/avr-window>
