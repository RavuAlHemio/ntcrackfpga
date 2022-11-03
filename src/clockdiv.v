// ECP5 eval board: X2 is 200 MHz
// nextpnr says:
// Info: Max frequency for clock '$glbnet$clk$TRELLIS_IO_IN': 82.95 MHz (PASS at 12.00 MHz)
// => divide by 3

module clockdiv(
    input fullclock,
    output reg divclock);

`ifdef SIMULATION

always #1 divclock = fullclock;

`else

// FIXME: use clockdiv on board

reg [1:0] counter;

initial begin
    counter <= 0;
end

always @ (posedge fullclock) begin
    if (counter == 2) begin
        counter <= 0;
        divclock <= 1;
    end else begin
        counter <= counter + 1;
        divclock <= 0;
    end
end

`endif

endmodule
