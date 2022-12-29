module pwadder_test();

reg nrst;
reg clk;
reg [3:0] current_test;
reg [2:0] state;

reg [159:0] in_password;
reg [4:0] in_length;
reg trigger;
wire [159:0] out_password;
wire [4:0] out_length;
wire completed;

reg [3:0] nrst_counter;

pwadder password_adder(
    .nrst(nrst),
    .clk(clk),
    .in_password(in_password),
    .in_length(in_length),
    .trigger(trigger),
    .out_password(out_password),
    .out_length(out_length),
    .completed(completed));

initial begin
    $dumpfile("pwadder_test.vcd");
    $dumpvars(0, pwadder_test);

    nrst <= 0;
    nrst_counter <= 0;

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

`include "gen/inc/slice_equality.v"

always @ (posedge clk) begin
    if (!nrst) begin
        if (nrst_counter == 4'b1111)
            nrst <= 1;
        else
            nrst_counter <= nrst_counter + 1;
    end else begin
        case (state)
            0, 5: begin
                case (current_test)
                    0: if (state == 0) begin
                        // prepare the test
                        in_password <= 160'h2020202020202020202020202020202020202020;
                        in_length <= 0;
                    end else begin
                        // check the result
                        check_test_result_1('h20);
                    end
                    1: if (state == 0) begin
                        in_password <= 160'h2020202020202020202020202020202020202020;
                        in_length <= 1;
                    end else begin
                        check_test_result_1('h21);
                    end
                    2: if (state == 0) begin
                        in_password <= 160'h2020202020202020202020202020202020202021;
                        in_length <= 1;
                    end else begin
                        check_test_result_1('h22);
                    end
                    3: if (state == 0) begin
                        in_password <= 160'h202020202020202020202020202020202020207D;
                        in_length <= 1;
                    end else begin
                        check_test_result_1('h7E);
                    end
                    4: if (state == 0) begin
                        in_password <= 160'h202020202020202020202020202020202020207E;
                        in_length <= 1;
                    end else begin
                        check_test_result_2('h2020);
                    end
                    5: if (state == 0) begin
                        in_password <= 160'h2020202020202020202020202020202020202021;
                        in_length <= 2;
                    end else begin
                        check_test_result_2('h2022);
                    end
                    6: if (state == 0) begin
                        in_password <= 160'h202020202020202020202020202020202020207E;
                        in_length <= 2;
                    end else begin
                        check_test_result_2('h2120);
                    end
                    7: if (state == 0) begin
                        in_password <= 160'h2020202020202020202020202020202054455354;
                        in_length <= 4;
                    end else begin
                        check_test_result_4('h54455355);
                    end
                    8: if (state == 0) begin
                        in_password <= 160'h20202020202020202020202020207E547E7E7E7E;
                        in_length <= 6;
                    end else begin
                        check_test_result_6('h7E5520202020);
                    end
                    9: if (state == 0) begin
                        in_password <= 160'h7E7E7E7E7E7E7E7E7E7E7E7E7E7E7E7E7E7E7E7E;
                        in_length <= 20;
                    end else begin
                        if (out_length !== 21) begin
                            $display("test 9 output length is not 21 but %d", out_length);
                            $finish;
                        end else if (out_password !== 160'h2020202020202020202020202020202020202020) begin
                            $display("test 9 output is not twenty time 'h20 but %020x", out_password);
                            $finish;
                        end else begin
                            $display("test 9 OK");
                        end
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
                if (current_test == 9) begin
                    $display("all tests completed");
                    $finish();
                end

                // next test
                current_test <= current_test + 1;
                state <= 0;
            end
        endcase
    end
end

endmodule
