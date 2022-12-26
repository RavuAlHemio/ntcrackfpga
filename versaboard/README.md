# Versa Board

The cracker itself is structured to run on a Lattice ECP5 Evaluation Board (`LFE5UM5G-85F-EVN`; relevant settings may be found in `src/ntcrackfpga.lpf` and `src/clockdiv.v`). To provide a more convenient interface without attempting to implement I2C or UART in Verilog (and failing miserably), a Microchip-SAM-based _Versa Board_ solution is implemented, which is meant to mate directly with the Versa pins on the evaluation board.

The centerpoint of the _Versa Board_ is a Microchip (ex-Atmel) SAML21G18 microcontroller.

A KiCad hardware design is provided in this directory; the software implementation (in Rust) is in the `software` subdirectory.

## SAML21 pinout

The following SAML21 pins are used for the interconnection with the evaluation board:

| SAML21 pin | ECP5 Eval Board | function           | direction | ECP5 ball |
| ---------- | --------------- | ------------------ | --------- | --------- |
| PB08       | J39 pin 19      | `store_hash_byte`  | →         | A9        |
| PB09       | J39 pin 20      | `go`               | →         | B10       |
| PA06       | J39 pin 29      | `match_found`      | ←         | E7        |
| PA07       | J39 pin 31      | `your_turn`        | ←         | A11       |
| PA08       | J40 pin 4       | `new_hash_byte[0]` | →         | F1        |
| PA09       | J40 pin 6       | `new_hash_byte[1]` | →         | G1        |
| PA10       | J40 pin 8       | `new_hash_byte[2]` | →         | J5        |
| PA11       | J40 pin 10      | `new_hash_byte[3]` | →         | K3        |
| PA12       | J40 pin 12      | `new_hash_byte[4]` | →         | L5        |
| PA13       | J40 pin 14      | `new_hash_byte[5]` | →         | N5        |
| PA14       | J40 pin 16      | `new_hash_byte[6]` | →         | P5        |
| PA15       | J40 pin 18      | `new_hash_byte[7]` | →         | M3        |
| PA16       | J40 pin 1       | `output_byte[0]`   | ←         | K2        |
| PA17       | J40 pin 5       | `output_byte[1]`   | ←         | H2        |
| PA18       | J40 pin 7       | `output_byte[2]`   | ←         | J4        |
| PA19       | J40 pin 9       | `output_byte[3]`   | ←         | J3        |
| PA20       | J40 pin 11      | `output_byte[4]`   | ←         | L4        |
| PA21       | J40 pin 13      | `output_byte[5]`   | ←         | M4        |
| PA22       | J40 pin 15      | `output_byte[6]`   | ←         | N4        |
| PA23       | J40 pin 17      | `output_byte[7]`   | ←         | N3        |
| PA25       | J40 pin 21      | `step_active`      | →         | K5        |
| PB23       | J40 pin 23      | `step_clock`       | →         | M5        |
| PB22       | J40 pin 25      | `state_give`       | →         | L3        |

The following pins are used for other functions:

| SAML21 pin | function                               |
| ---------- | -------------------------------------- |
| PA00       | XIN32 (32kHz oscillator input)         |
| PA01       | reserved for XOUT32 (32k osc. output)  |
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
