module hashchecker_test();

reg nrst;
reg clk;
reg newrdy;
reg checkrdy;
reg [127:0] hash;
wire resultrdy;
wire matchfound;

reg [3:0] current_index;
reg [3:0] state;

reg [3:0] nrst_counter;

hashchecker checker(
    .nrst(nrst),
    .clk(clk),
    .newrdy(newrdy),
    .checkrdy(checkrdy),
    .hash(hash),
    .resultrdy(resultrdy),
    .matchfound(matchfound));

initial begin
    $dumpfile("hashchecker_test.vcd");
    $dumpvars(0, hashchecker_test);

    // activate NRST
    nrst <= 0;
    nrst_counter <= 0;

    clk <= 0;
    newrdy <= 0;
    checkrdy <= 0;
    hash <= 0;

    current_index <= 0;
    state <= 0;
end

always begin
    #1
    clk <= !clk;
end

`define ASSERT_FOUND \
    if (matchfound) begin \
        $display("test %1d OK", current_index); \
    end else begin \
        $display("test %1d failed -- did not find expected hash", current_index); \
        $finish; \
    end
`define ASSERT_NOT_FOUND \
    if (matchfound) begin \
        $display("test %1d failed -- found unexpected hash", current_index); \
        $finish; \
    end else begin \
        $display("test %1d OK", current_index); \
    end

always @ (posedge clk) begin
    if (!nrst) begin
        if (nrst_counter == 4'b1111)
            nrst <= 1;
        else
            nrst_counter <= nrst_counter + 1;
    end else begin
        case (state)
            // preparation block
            0: begin
                // prepare a hash to store
                case (current_index)
                    0: hash <= 128'h0CB6948805F797BF2A82807973B89537; // md4(utf6le("test"))
                    1: hash <= 128'h7454070F0339BBC993CB08EAF741513A; // md4(utf6le("reindeerflotilla"))
                    2: hash <= 128'h61FB34469B9989B01BE4E8630C52EED6; // md4(utf6le("swordfish"))
                endcase

                state <= 1;
            end
            1: begin
                // ask the checker to store it
                newrdy <= 1;

                state <= 2;
            end
            2: begin
                // hold...

                state <= 3;
            end
            3: begin
                // enough storing
                newrdy <= 0;

                state <= 4;
            end
            4: begin
                // increment hash index
                current_index <= current_index + 1;

                if (current_index == 3) begin
                    // we're done storing hashes
                    current_index <= 0;
                    state <= 5;
                end else begin
                    // store the next hash
                    state <= 0;
                end
            end

            // testing block
            5, 10: begin
                case (current_index)
                    0: if (state == 5) begin
                        // try finding "test"
                        hash <= 128'h0CB6948805F797BF2A82807973B89537;
                    end else begin
                        // should have been found
                        `ASSERT_FOUND
                    end
                    1: if (state == 5) begin
                        // "swordfish"
                        hash <= 128'h61FB34469B9989B01BE4E8630C52EED6;
                    end else begin
                        `ASSERT_FOUND
                    end
                    2: if (state == 5) begin
                        // "1234"
                        hash <= 128'h7CE21F17C0AEE7FB9CEBA532D0546AD6;
                    end else begin
                        `ASSERT_NOT_FOUND
                    end
                endcase

                if (state == 5)
                    state <= 6;
                else
                    state <= 11;
            end
            6: begin
                // trigger the test
                checkrdy <= 1;

                state <= 7;
            end
            7: begin
                // leave the test triggered
                state <= 8;
            end
            8: begin
                // untrigger the test
                checkrdy <= 0;

                state <= 9;
            end
            9: begin
                // wait for test completion
                if (resultrdy)
                    state <= 10;
            end
            // 10 has been moved up to be close to the test definition
            11: begin
                if (current_index == 2) begin
                    $display("all tests completed");
                    $finish();
                end

                // next test
                current_index <= current_index + 1;
                state <= 5;
            end
        endcase
    end
end

endmodule
