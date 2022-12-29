module ntcrackfpga_hw(
    input [7:0] new_hash_byte,
    input store_hash_byte,
    input go,
    input step_active,
    input give_state,
    input step_clock_bouncy,
    input nrst_bouncy,
    output reg match_found,
    output reg your_turn,
    output wire [7:0] output_byte);

wire nrst;
wire builtin_clock;
wire output_clock;
wire [7:0] stater_state_byte;
wire [4:0] password_len;
wire [159:0] password_chars;
wire [(128*128-1):0] hashes;
wire [127:0] md4_hash_holder;
wire [4:0] ntcrackfpga_state;
wire [3:0] hashchecker_state;
wire [5:0] md4block_step;

wire cracker_clock;
wire stater_clock;
reg step_clock;
reg [7:0] password_byte;

// nextpnr estimates our max clock at 70 MHz
// base clock is ~310 MHz
// => use a divisor of 5 (~62MHz)
OSCG #(.DIV(5)) built_in_clock(.OSC(builtin_clock));

clockpick clockpicker(
    builtin_clock,
    step_clock,
    step_active,
    output_clock);

ntcrackfpga cracker(
    .nrst(nrst),
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
    .md4_hash_holder(md4_hash_holder),
    .state(ntcrackfpga_state),
    .md4block_step(md4block_step),
    .hashchecker_state(hashchecker_state));

state_giver stater(
    .nrst(nrst),
    .clk(stater_clock),
    .password_len(password_len),
    .password_chars(password_chars),
    .hashes(hashes),
    .current_hash(md4_hash_holder),
    .ntcrackfpga_state(ntcrackfpga_state),
    .hashchecker_state(hashchecker_state),
    .md4block_step(md4block_step),
    .state_byte(stater_state_byte));

debounce nrst_debouncer(
    .clk(builtin_clock),
    .in(nrst_bouncy),
    .out(nrst));

debounce #(.inverting(1'b1)) step_clock_debouncer(
    .clk(builtin_clock),
    .in(step_clock_bouncy),
    .out(step_clock));

assign stater_clock = give_state ? output_clock : 0;
assign cracker_clock = give_state ? 0 : output_clock;
assign output_byte = give_state ? stater_state_byte : password_byte;

endmodule
