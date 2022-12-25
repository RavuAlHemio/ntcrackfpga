module clockpick(
    input regular_clock,
    input step_clock,
    input step_active,
    output wire out_clock);

assign out_clock = step_active ? step_clock : regular_clock;

endmodule
