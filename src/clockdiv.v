// ECP5 eval board: X2 is 200 MHz
// nextpnr says:
// Info: Max frequency for clock '$glbnet$clk$TRELLIS_IO_IN': 82.95 MHz (PASS at 12.00 MHz)
// => divide by 3

module clockdiv(
    input fullclock,
    output reg divclock);

reg [1:0] counter;

always @ (posedge fullclock) begin
    if (counter == 2) begin
        counter <= 0;
        divclock <= 1;
    end else begin
        counter <= counter + 1;
        divclock <= 0;
    end
end

endmodule
