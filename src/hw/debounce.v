module debounce #(
    parameter [0:0] inverting = 0) (
    input clk,
    input in,
    output reg out);

reg [7:0] counter;

initial begin
    counter <= 0;
    out <= 0;
end

`define UPWARD \
    if (counter == 8'hFF) \
        out <= 1; \
    else \
        counter <= counter + 1;

`define DOWNWARD \
    if (counter == 8'h00) \
        out <= 0; \
    else \
        counter <= counter - 1;

always @ (posedge clk) begin
    if (inverting) begin
        if (in) `UPWARD else `DOWNWARD
    end else begin
        if (in) `DOWNWARD else `UPWARD
    end
end

endmodule
