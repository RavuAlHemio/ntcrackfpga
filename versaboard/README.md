# Versa Board

The cracker itself is structured to run on a Lattice ECP5 Evaluation Board (`LFE5UM5G-85F-EVN`; relevant settings may be found in `src/ntcrackfpga.lpf` and `src/clockdiv.v`). To provide a more convenient interface without attempting to implement I2C or UART in Verilog (and failing miserably), a Microchip-SAM-based _Versa Board_ solution is implemented, which is meant to mate directly with the Versa pins on the evaluation board.

The centerpoint of the _Versa Board_ is a Microchip (ex-Atmel) SAML21G18 microcontroller.

A KiCad hardware design is provided in this directory; the software implementation (in Rust) is in the `software` subdirectory.
