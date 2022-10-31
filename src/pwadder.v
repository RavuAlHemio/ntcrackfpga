module pwadder(
    input clk,
    input [159:0] in_password,
    input [5:0] in_length,
    input trigger,
    output reg [159:0] out_password,
    output reg [5:0] out_length,
    output completed);

reg running;
reg index;
reg signal_completion;

initial begin
    running <= 0;
    index <= 0;
end

always @ (posedge clk) begin
    if (running) begin
        if (signal_completion) begin
            if (completed) begin
                completed <= 0;
                signal_completion <= 0;
                running <= 0;
            end else begin
                completed <= 1;
            end
        end else if (in_length == 0) begin
            // this is rather trivial
            out_password <= {8'h20, 0};
            out_length <= 1;
            signal_completion <= 1;
        end else begin
            // TODO: get value of in_password[index]
            // TODO: increment by 1
            // TODO: on overflow, continue with next index
            // TODO: if no overflow, copy other bytes and signal completion
            // TODO: if we run out of indices, increment length and start with 0x202020...
        end
    end else begin
        // currently not running
        if (trigger) begin
            running <= 1;
            out_password <= in_password;
            out_length <= in_length;
            index <= in_length - 1;
        end
    end
end

always @ (posedge trigger) begin
    if (in_byte == 8'h7E) begin
        out_byte <= 8'h20;
        carry <= 1;
    end else begin
        out_byte <= in_byte + 1;
        carry <= 0;
    end
end

endmodule
