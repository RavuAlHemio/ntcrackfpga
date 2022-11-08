module clockdiv(
    input fullclock,
    output reg divclock);

`ifdef SIMULATION

// no clock division
always #1 divclock = fullclock;

`else

// ECP5 eval board: X2 is 200 MHz
// nextpnr says:
// Info: Max frequency for clock '$glbnet$div_clk': 69.95 MHz (PASS at 12.00 MHz)
// => divide by 3.5 (57.14 MHz)

// use clockdivs on board
CLKDIVF #(.DIV("3.5")) internal_cdiv0(
    .CLKI(fullclock),
    .RST(1'b0),
    .ALIGNWD(1'b0),
    .CDIVX(divclock));

`endif

endmodule
