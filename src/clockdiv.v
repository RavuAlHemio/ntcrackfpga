module clockdiv(
    input fullclock,
    output reg divclock);

`ifdef SIMULATION

// no clock division
always #1 divclock = fullclock;

`else

// ECP5 eval board: X2 is 200 MHz
// nextpnr says:
// Info: Max frequency for clock '$glbnet$clk$TRELLIS_IO_IN': 82.95 MHz (PASS at 12.00 MHz)
// => divide by (at least) 3

wire passthrough1;
wire passthrough2;

// use clockdivs on board
CLKDIVF #(.DIV("3.5")) internal_cdiv0(
    .CLKI(fullclock),
    .RST(1'b0),
    .ALIGNWD(1'b0),
    .CDIVX(passthrough1));
CLKDIVF #(.DIV("3.5")) internal_cdiv1(
    .CLKI(passthrough1),
    .RST(1'b0),
    .ALIGNWD(1'b0),
    .CDIVX(passthrough2));
CLKDIVF #(.DIV("3.5")) internal_cdiv2(
    .CLKI(passthrough2),
    .RST(1'b0),
    .ALIGNWD(1'b0),
    .CDIVX(divclock));

`endif

endmodule
