module pwadder_test();

reg clk;
reg current_test;
reg [2:0] state;

reg [159:0] in_password;
reg [4:0] in_length;
reg trigger;
wire [159:0] out_password;
wire [4:0] out_length;
wire completed;

pwadder password_adder(
    clk,
    in_password,
    in_length,
    trigger,
    out_password,
    out_length,
    completed);

initial begin
    $dumpfile("pwadder_test.vcd");
    $dumpvars(0, pwadder_test);

    clk <= 0;
    current_test <= 0;
    state <= 0;
    in_password <= 160'h2020202020202020202020202020202020202020;
    in_length <= 0;
    trigger <= 0;
end

always begin
    #1
    clk <= !clk;
end

task check_test_result(
    input [159:0] expected_out_password,
    input [4:0] expected_out_length);
        if (out_password != expected_out_password) begin
            $display("test %d failed on password! expected %040x, obtained %040x", current_test, expected_out_password, out_password);
            $finish();
        end else if (out_length != expected_out_length) begin
            $display("test %d failed on length! expected %02x, obtained %02x", current_test, expected_out_length, out_length);
            $finish();
        end else begin
            $display("test %d OK", current_test);
        end
endtask

always @ (posedge clk) begin
    case (state)
        0, 5: begin
            case (current_test)
                0: if (state == 0) begin
                    // prepare the test
                    in_password <= 160'h2020202020202020202020202020202020202020;
                    in_length <= 0;
                end else begin
                    // check the result
                    check_test_result({8'h20, 152'hX}, 1);
                end
            endcase

            if (state == 0)
                state <= 1;
            else
                state <= 6;
        end
        1: begin
            // trigger the test
            trigger <= 1;

            state <= 2;
        end
        2: begin
            // leave the test triggered
            state <= 3;
        end
        3: begin
            // untrigger the test
            trigger <= 0;

            state <= 4;
        end
        4: begin
            // wait for test completion
            if (completed)
                state <= 5;
        end
        // 5 has been moved up to be close to the test definition
        6: begin
            if (current_test == 0) begin
                $display("all tests completed");
                $finish();
            end

            // next test
            current_test <= current_test + 1;
            state <= 0;
        end
    endcase
end

endmodule
