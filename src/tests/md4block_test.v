module md4block_test();

reg clk;
reg irdy;
reg [31:0] state_a, state_b, state_c, state_d;
reg [511:0] data;
wire ordy;
wire [31:0] newstate_a, newstate_b, newstate_c, newstate_d;

reg [3:0] current_index;
reg [3:0] state;
reg [127:0] final_hash;

md4block md4(
    clk,
    irdy,
    state_a,
    state_b,
    state_c,
    state_d,
    data,
    ordy,
    newstate_a,
    newstate_b,
    newstate_c,
    newstate_d);

`include "src/inc/byteswap.v"
`include "src/inc/md4constants.v"

initial begin
    $dumpfile("md4block_test.vcd");
    $dumpvars(0, md4block_test);

    clk <= 0;
    irdy <= 0;
    state_a <= 0;
    state_b <= 0;
    state_c <= 0;
    state_d <= 0;
    data <= 0;

    current_index <= 0;
    state <= 0;
end

always begin
    #1
    clk <= !clk;
end

always @ (posedge clk) begin
    case (state)
        0: begin
            // populate a block
            case (current_index)
                // each is split into: password, padding byte, zero padding, length value (if any), leading zeroes of length
                0: data <= {/* 0'h0, */ 8'h80, 440'h0, 64'h0}; // md4padding("")
                1: data <= {32'h74657374, 8'h80, 408'h0, 8'h20, 56'h0}; // md4padding("test")
                2: data <= {128'h7265696E64656572666C6F74696C6C61, 8'h80, 312'h0, 8'h80, 56'h0}; // md4padding("reindeerflotilla")
                3: data <= {72'h73776F726466697368, 8'h80, 368'h0, 8'h48, 56'h0}; // md4padding("swordfish")
            endcase

            // also, prepare MD4
            `SET_MD4_INITIAL(state_a, state_b, state_c, state_d);

            state <= 1;
        end
        6: begin
            // compare test results
            `define EVALUATE_TEST(expected_hash) begin \
                    if (final_hash == expected_hash) begin \
                        $display("test %1d OK", current_index); \
                    end else begin \
                        $display("test %1d hash mismatch: %032x != %032x", current_index, final_hash, expected_hash); \
                        $finish; \
                    end \
                end

            case (current_index)
                0: `EVALUATE_TEST(128'h31D6CFE0D16AE931B73C59D7E0C089C0)
                1: `EVALUATE_TEST(128'hDB346D691D7ACC4DC2625DB19F9E3F52)
                2: `EVALUATE_TEST(128'hDEABAE991701C6BEECB3949552F07601)
                3: `EVALUATE_TEST(128'h5E2047B913668435800AB70F839F62AB)
            endcase

            state <= 7;
        end

        1: begin
            // ask the block to start
            irdy <= 1;

            state <= 2;
        end
        2: begin
            // hold...

            state <= 3;
        end
        3: begin
            // enough starting
            irdy <= 0;

            state <= 4;
        end
        4: begin
            // wait until MD4 is done
            if (ordy)
                state <= 5;
        end
        5: begin
            // update local register
            final_hash <= byteswap_md4(newstate_a, newstate_b, newstate_c, newstate_d);

            state <= 6;
        end
        // 6: moved upward to be closer to input data
        7: begin
            if (current_index == 3) begin
                // we are done
                $finish();
            end

            // increment hash index
            current_index <= current_index + 1;

            // start at the beginning
            state <= 0;
        end
    endcase
end

endmodule
