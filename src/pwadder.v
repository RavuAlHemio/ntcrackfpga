module pwadder(
    input clk,
    input [159:0] in_password,
    input [4:0] in_length,
    input trigger,
    output reg [159:0] out_password,
    output reg [4:0] out_length,
    output reg completed);

reg [3:0] state;
reg [4:0] index;
reg read_trigger;
reg write_trigger;
wire [7:0] read_slice;
reg [7:0] write_slice;
wire [159:0] write_out_password;

mux_read_8_of_160 mux_reader(
    in_password,
    index,
    read_trigger,
    read_slice);
mux_write_8_of_160 mux_writer(
    write_out_password,
    index,
    write_trigger,
    write_slice);

initial begin
    state <= 0;
    index <= 0;
end

always @ (posedge clk) begin
    case (state)
        0: begin
            // dormant
            if (trigger) begin
                out_password <= in_password;
                out_length <= in_length;
                index <= in_length - 1;
                state <= 1;
            end
        end
        1: begin
            if (in_length == 0) begin
                // this is rather trivial
                out_password <= {8'h20, 152'h0};
                out_length <= 1;
                state <= 9; // we can signal completion
            end else begin
                // get value of in_password[index]
                read_trigger <= 1;

                state <= 2;
            end
        end
        2: begin
            // keep read_trigger triggered

            state <= 3;
        end
        3: begin
            // untrigger read_trigger
            read_trigger <= 0;

            state <= 4;
        end
        4: begin
            // increment
            if (read_slice == 8'h7E) begin
                if (index == 0) begin
                    // increment is overflowing; increase the length
                    out_password <= 160'h2020202020202020202020202020202020202020;
                    out_length <= out_length + 1;
                    state <= 9; // we can signal completion
                end else begin
                    // try the next index to the left
                    write_slice <= 8'h20;
                    index <= index - 1;
                    state <= 5;
                end
            end
        end
        5: begin
            // trigger write
            write_trigger <= 1;

            state <= 6;
        end
        6: begin
            // keep write_trigger triggered

            state <= 7;
        end
        7: begin
            // untrigger write
            write_trigger <= 0;

            state <= 8;
        end
        8: begin
            // transfer the password from the writer to our output
            out_password <= write_out_password;

            if (read_slice == 8'h7E)
                // we are carrying; increment the next index
                state <= 1;
            else
                // we are done; start signalling completion
                state <= 9;
        end
        9: begin
            // start signalling completion
            completed <= 1;

            state <= 10;
        end
        10: begin
            // keep signalling completion

            state <= 11;
        end
        11: begin
            // stop signalling completion
            completed <= 0;

            // return to dormancy
            state <= 0;
        end
    endcase
end

endmodule
