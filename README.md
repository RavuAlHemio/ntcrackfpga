# ntcrackfpga

FPGA-based NT hash cracker.

NT hashes are universally `MD4(UTF16LE(password))`. If you think this sounds like low-hanging fruit, that's because it is.

The reference implementation runs on a Lattice ECP5 Evaluation Board (`LFE5UM5G-85F-EVN`) while tests are simulated using Icarus Verilog. However, the Verilog code should be generic enough to work anywhere; device-specific settings are limited to `src/ntcrackfpga.lpf` and `src/clockdiv.v`.

See `PROTOCOL.md` on how to communicate with the cracker.
