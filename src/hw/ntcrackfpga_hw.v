module ntcrackfpga_hw(
    input [7:0] new_hash_byte,
    input store_hash_byte,
    input go,
    input step_active,
    input give_state,
    input step_clock_bouncy,
    output reg match_found,
    output reg your_turn,
    output wire [7:0] output_byte);

wire builtin_clock;
wire output_clock;
wire [7:0] stater_state_byte;
wire [4:0] password_len;
wire [159:0] password_chars;
wire [(128*128-1):0] hashes;
wire [127:0] md4_hash_holder;

wire cracker_clock;
wire stater_clock;
reg step_clock;
reg [7:0] password_byte;

// nextpnr estimates our max clock at 70 MHz
// base clock is ~310 MHz
// => we could use a divisor of 5 (~62MHz)
// be conservative and use 8 (~38MHz)
OSCG #(.DIV(8)) built_in_clock(.OSC(builtin_clock));

clockpick clockpicker(
    builtin_clock,
    step_clock,
    step_active,
    output_clock);

ntcrackfpga cracker(
    .clk(cracker_clock),
    .new_hash_byte(new_hash_byte),
    .store_hash_byte(store_hash_byte),
    .go(go),
    .match_found(match_found),
    .your_turn(your_turn),
    .password_byte(password_byte),
    .password_len(password_len),
    .password_chars(password_chars),
    .hashes(hashes),
    .md4_hash_holder(md4_hash_holder));

state_giver stater(
    stater_clock,
    password_len,
    password_chars,
    hashes,
    md4_hash_holder,
    stater_state_byte);

debounce #(.inverting(1'b1)) step_clock_debouncer(
    builtin_clock,
    step_clock_bouncy,
    step_clock);

assign stater_clock = give_state ? output_clock : 0;
assign cracker_clock = give_state ? 0 : output_clock;
assign output_byte = give_state ? stater_state_byte : password_byte;

endmodule
