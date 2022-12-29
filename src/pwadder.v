module pwadder(
    input nrst,
    input clk,
    input [159:0] in_password,
    input [4:0] in_length,
    input trigger,
    output reg [159:0] out_password,
    output reg [4:0] out_length,
    output reg completed);

`include "gen/inc/muxes.v"

reg [3:0] state;
reg [4:0] index;
reg [7:0] slice;
reg carry;

always @ (posedge clk) begin
    if (!nrst) begin
        state <= 0;
        index <= 0;
        carry <= 0;
    end else begin
        case (state)
            0: begin
                // dormant
                if (trigger) begin
                    out_password <= in_password;
                    out_length <= in_length;
                    index <= 0;
                    state <= 1;
                end
            end
            1: begin
                if (in_length == 0) begin
                    // this is rather trivial
                    out_password <= {152'h0, 8'h20};
                    out_length <= 1;
                    state <= 6; // completion
                end else begin
                    // get value of in_password[index]
                    `MUX_READ_8_OF_160(in_password, index, slice);

                    state <= 2;
                end
            end
            2: begin
                // increment
                if (slice == 8'h7E) begin
                    // carry
                    slice <= 8'h20;
                    carry <= 1;
                end else begin
                    // regular increment
                    slice <= slice + 1;
                end
                state <= 3;
            end
            3: begin
                // write value
                `MUX_WRITE_8_OF_160(out_password, index, slice);

                state <= 4;
            end
            4: begin
                // handle next step
                if (carry) begin
                    // we are carrying; increment the next index
                    carry <= 0;
                    index <= index + 1;
                    state <= 5;
                end else begin
                    // we are done; start signalling completion
                    state <= 6; // completion
                end
            end
            5: begin
                // we are still carrying; halt before the final overflow
                if (index == in_length) begin
                    // we have gone through all passwords of this length
                    // reset to zero and signal an increase
                    out_password <= 160'h2020202020202020202020202020202020202020;
                    out_length <= in_length + 1;
                    state <= 6; // completion
                end else begin
                    // go around
                    state <= 1;
                end
            end
            6: begin
                // start signalling completion
                completed <= 1;

                state <= 7;
            end
            7: begin
                // keep signalling completion

                state <= 8;
            end
            8: begin
                // stop signalling completion
                completed <= 0;

                // return to dormancy
                state <= 0;
            end
        endcase
    end
end

endmodule
