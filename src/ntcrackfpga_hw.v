module ntcrackfpga_hw(
    input [7:0] new_hash_byte,
    input store_hash_byte,
    input go,
    output reg match_found,
    output reg your_turn,
    output reg [7:0] password_byte,
    output reg blinky_led);

wire clk;

// nextpnr estimates our max clock at 70 MHz
// base clock is ~310 MHz
// => we could use a divisor of 5 (~62MHz)
// be conservative and use 8 (~38MHz)
OSCG #(.DIV(8)) built_in_clock(.OSC(clk));

ntcrackfpga cracker(
    clk,
    new_hash_byte,
    store_hash_byte,
    go,
    match_found,
    your_turn,
    password_byte,
    blinky_led);

endmodule
