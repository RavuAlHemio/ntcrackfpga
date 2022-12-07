# Versa Board

The cracker itself is structured to run on a Lattice ECP5 Evaluation Board (`LFE5UM5G-85F-EVN`; relevant settings may be found in `src/ntcrackfpga.lpf` and `src/clockdiv.v`). To provide a more convenient interface without attempting to implement I2C or UART in Verilog (and failing miserably), a Microchip-SAM-based _Versa Board_ solution is implemented, which is meant to mate directly with the Versa pins on the evaluation board.

The centerpoint of the _Versa Board_ is a Microchip (ex-Atmel) SAML21G18 microcontroller.

A KiCad hardware design is provided in this directory; the software implementation (in Rust) is in the `software` subdirectory.

## SAML21 pinout

The following SAML21 pins are used for the interconnection with the evaluation board:

| SAML21 pin | ECP5 Eval Board | function           | direction | ECP5 ball |
| ---------- | --------------- | ------------------ | --------- | --------- |
| PB08       | J39 pin 4       | `store_hash_byte`  | →         | D15       |
| PB09       | J39 pin 6       | `go`               | →         | C15       |
| PA06       | J39 pin 8       | `match_found`      | ←         | B20       |
| PA07       | J39 pin 10      | `your_turn`        | ←         | E11       |
| PA08       | J39 pin 5       | `new_hash_byte[0]` | →         | B15       |
| PA09       | J39 pin 7       | `new_hash_byte[1]` | →         | B13       |
| PA10       | J39 pin 9       | `new_hash_byte[2]` | →         | D11       |
| PA11       | J39 pin 11      | `new_hash_byte[3]` | →         | B12       |
| PA12       | J39 pin 13      | `new_hash_byte[4]` | →         | D12       |
| PA13       | J39 pin 15      | `new_hash_byte[5]` | →         | C13       |
| PA14       | J39 pin 17      | `new_hash_byte[6]` | →         | E13       |
| PA15       | J39 pin 19      | `new_hash_byte[7]` | →         | A9        |
| PA16       | J40 pin 1       | `password_byte[0]` | ←         | K2        |
| PA17       | J40 pin 3       | `password_byte[1]` | ←         | A15       |
| PA18       | J40 pin 5       | `password_byte[2]` | ←         | H2        |
| PA19       | J40 pin 7       | `password_byte[3]` | ←         | J4        |
| PA20       | J40 pin 9       | `password_byte[4]` | ←         | J3        |
| PA21       | J40 pin 11      | `password_byte[5]` | ←         | L4        |
| PA22       | J40 pin 13      | `password_byte[6]` | ←         | M4        |
| PA23       | J40 pin 15      | `password_byte[7]` | ←         | N4        |

The following pins are used for other functions:

| SAML21 pin | function                               |
| ---------- | -------------------------------------- |
| PA00       | XIN32 (32kHz oscillator input)         |
| PA01       | XOUT32 (32kHz oscillator output)       |
| PA02       | avoided to ensure oscillator stability |
| PA03       | avoided to ensure oscillator stability |
| PA04       | UART TX (board to PC)                  |
| PA05       | UART RX (PC to board)                  |
| PA24       | on-board LED                           |
| PA30       | SWCLK (debugging, clock signal)        |
| PA31       | SWDIO (debugging, I/O)                 |
| PB02       | avoided to ensure oscillator stability |
| PB03       | avoided to ensure oscillator stability |

## JLCPCB notes

If you wish to have this board manufactured by JLCPCB, please note the following rotation differences:

| components | description     | JLCPCB rotation        |
| ---------- | --------------- | ---------------------- |
| J1         | UART pins       | 90°                    |
| J4         | debug header    | 90°                    |
| J39, J40   | pin socket      | 270°                   |
| U1         | microcontroller | 270°? (to be verified) |

JLCPCB-specific rotations can be specified in the [KiCad JLCPCB Tools plugin](https://github.com/Bouni/kicad-jlcpcb-tools).
