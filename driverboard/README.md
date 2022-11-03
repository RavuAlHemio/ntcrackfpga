# Driver Board

The cracker itself is structured to run on a Lattice ECP5 Evaluation Board (`LFE5UM5G-85F-EVN`; relevant settings may be found in `src/ntcrackfpga.lpf` and `src/clockdiv.v`). To provide a more convenient interface without attempting to implement I2C or UART in Verilog (and failing miserably), an Arduino-Zero-based _Driver Board_ solution is implemented.

Ingredients:

* Lattice ECP5 Evaluation Board (`LFE5UM5G-85F-EVN`; runs the cracker)
* Arduino Zero (`ABX00003`; used instead of the Uno for its 3.3V logic levels)
* Mikroelektronika Expand 12 Click (`MIKROE-4889`; port expander)

Connections between Arduino and Expand 12 Click:

| Arduino                             | Expand 12 Click                               |
| ----------------------------------- | --------------------------------------------- |
| SCL (top header, leftmost)          | SCL (right mikroBUS header, 5th from top)     |
| SDA (top header, 2nd from left)     | SDA (right mikroBUS header, 6th from top)     |
| 3.3V (bottom header, 4th from left) | +3.3V (left mikroBUS header, 2nd from bottom) |
| GND (bottom header, 6th from left)  | GND (either mikroBUS header, bottommost)      |

Connections between Expand 12 Click and ECP5 Evaluation Board:

| Expand 12 Click | dir | ECP5 Eval Board              | usage                 |
| --------------- | --- | ---------------------------- | --------------------- |
| pin 4           | →   | Versa J39 pin 4              | `store_hash_byte`     |
| pin 5           | →   | Versa J39 pin 6              | `go`                  |
| pin 6           | ←   | Versa J39 pin 8              | `match_found`         |
| pin 7           | ←   | Versa J39 pin 10             | `your_turn`           |
| pins 8 to 15    | →   | Versa J39 pins 5, 7, ..., 19 | `new_hash_byte[0..7]` |
| pins 16 to 23   | ←   | Versa J40 pins 1, 3, ..., 15 | `password_byte[0..7]` |
