module ntcrackfpga(
    input clk,
    input [7:0] new_hash_byte,
    input store_hash_byte,
    input go,
    output reg match_found,
    output reg your_turn,
    output reg [7:0] password_byte,

    // debug outputs
    output reg [4:0] password_len,
    output reg [159:0] password_chars,
    output wire [(128*128-1):0] hashes,
    output reg [127:0] md4_hash_holder,
    output reg [4:0] state,
    output wire [5:0] md4block_step,
    output wire [3:0] hashchecker_state);

`include "gen/inc/encodepwd.v"
`include "src/inc/byteswap.v"
`include "src/inc/md4constants.v"

wire [159:0] next_password_chars;
wire [4:0] next_password_len;
reg increment_password_trigger;
wire increment_password_done;

reg md4_irdy;
reg [31:0] md4_in_a;
reg [31:0] md4_in_b;
reg [31:0] md4_in_c;
reg [31:0] md4_in_d;
reg [511:0] md4_data;
wire md4_ordy;
wire [31:0] md4_out_a;
wire [31:0] md4_out_b;
wire [31:0] md4_out_c;
wire [31:0] md4_out_d;

reg really_checking;

reg checker_newrdy;
reg checker_checkrdy;
wire checker_resultrdy;
wire checker_matchfound;

reg [3:0] next_hash_byte;
reg [4:0] password_byte_index;

pwadder passadder(
    clk,
    password_chars,
    password_len,
    increment_password_trigger,
    next_password_chars,
    next_password_len,
    increment_password_done);

md4block md4maker(
    clk,
    md4_irdy,
    md4_in_a,
    md4_in_b,
    md4_in_c,
    md4_in_d,
    md4_data,
    md4_ordy,
    md4_out_a,
    md4_out_b,
    md4_out_c,
    md4_out_d,
    md4block_step);

hashchecker hchecker(
    clk,
    checker_newrdy,
    checker_checkrdy,
    md4_hash_holder,
    checker_resultrdy,
    checker_matchfound,
    hashes,
    hashchecker_state);

initial begin
    match_found <= 0;
    your_turn <= 0;
    password_byte <= 0;

    state <= 0;

    password_chars <= 160'h2020202020202020202020202020202020202020;
    password_len <= 0;
    increment_password_trigger <= 0;

    md4_irdy <= 0;
    md4_in_a <= 0;
    md4_in_b <= 0;
    md4_in_c <= 0;
    md4_in_d <= 0;
    md4_data <= 0;

    md4_hash_holder <= 0;
    really_checking <= 0;

    checker_newrdy <= 0;
    checker_checkrdy <= 0;

    next_hash_byte <= 0;
    password_byte_index <= 0;

    your_turn <= 1;
end

always @ (posedge clk) begin
    case (state)
        // loading stage
        0: begin
            // wait until we are asked to store a hash byte or to start processing
            if (go) begin
                your_turn <= 0;
                state <= 7;
            end else if (store_hash_byte) begin
                your_turn <= 0;
                state <= 1;
            end
        end
        1: begin
            // store the prepared hash byte
            `MUX_WRITE_8_OF_128(md4_hash_holder, next_hash_byte, new_hash_byte);
            state <= 2;
        end
        2: begin
            // wait for store_hash_byte to be lowered again
            if (!store_hash_byte) begin
                state <= 3;
            end
        end
        3: begin
            if (next_hash_byte == 15) begin
                // the hash is complete; store it
                checker_newrdy <= 1;

                next_hash_byte <= 0;
                state <= 4;
            end else begin
                // wait for the next byte
                next_hash_byte <= next_hash_byte + 1;
                your_turn <= 1;

                state <= 0;
            end
        end
        4: begin
            // keep newrdy raised...

            state <= 5;
        end
        5: begin
            // pull newrdy down
            checker_newrdy <= 0;

            state <= 6;
        end
        6: begin
            // wait until the checker has stored the hash, then return back
            if (checker_resultrdy) begin
                your_turn <= 1;
                state <= 0;
            end
        end

        // processing stage
        7: begin
            // wait for go to be lowered again
            if (!store_hash_byte) begin
                state <= 8;
            end
        end
        8: begin
            // load up the current password
            `SET_MD4_INITIAL(md4_in_a, md4_in_b, md4_in_c, md4_in_d);
            md4_data <= password_to_md4_data(password_chars, password_len);

            state <= 9;
        end
        9: begin
            // trigger MD4
            md4_irdy <= 1;

            state <= 10;
        end
        10: begin
            // keep MD4 trigger up

            state <= 11;
        end
        11: begin
            // take down MD4 trigger
            md4_irdy <= 0;

            state <= 12;
        end
        12: begin
            // wait for MD4 to finish
            if (md4_ordy)
                state <= 13;
        end
        13: begin
            // make the MD4 state a hash
            md4_hash_holder <= byteswap_md4(md4_out_a, md4_out_b, md4_out_c, md4_out_d);
            state <= 14;
        end
        14: begin
            // let the hash database think about it
            checker_checkrdy <= 1;
            state <= 15;
        end
        15: begin
            // keep it up for a bit
            state <= 16;
        end
        16: begin
            // pull it down
            checker_checkrdy <= 0;
            state <= 17;
        end
        17: begin
            // wait until it's done
            if (checker_resultrdy)
                state <= 18;
        end
        18: begin
            // have we found anything?
            if (checker_matchfound) begin
                // go into elaboration mode
                password_byte_index <= 0;
                state <= 19;
            end else begin
                // go directly to password incrementing
                state <= 24;
            end
        end

        // elaboration mode
        19: begin
            // prepare a character of the password
            if (password_byte_index == 20) begin
                // send the length as the last byte
                password_byte <= {3'h0, password_len};
            end else begin
                // send that byte of the password
                `MUX_READ_8_OF_160(password_chars, password_byte_index, password_byte);
            end
            state <= 20;
        end
        20: begin
            // raise the "byte is ready" flag
            your_turn <= 1;
            match_found <= 1;
            state <= 21;
        end
        21: begin
            // keep match_found raised until "go"
            if (go) begin
                your_turn <= 0;
                match_found <= 0;
                state <= 22;
            end
        end
        22: begin
            // wait for "go" to be lowered again
            if (!go)
                state <= 23;
        end
        23: begin
            if (password_byte_index == 20) begin
                // this password is done; get on cracking
                state <= 24;
            end else begin
                // go to the next byte in elaboration mode
                password_byte_index <= password_byte_index + 1;
                state <= 19;
            end
        end

        // incrementation
        24: begin
            // increment the password
            increment_password_trigger <= 1;
            state <= 25;
        end
        25: begin
            // keep the flag up
            state <= 26;
        end
        26: begin
            // lower the flag
            increment_password_trigger <= 0;
            state <= 27;
        end
        27: begin
            // wait for incrementor to finish
            if (increment_password_done)
                state <= 28;
        end
        28: begin
            // transfer incremented password and restart processing stage
            password_chars <= next_password_chars;
            password_len <= next_password_len;
            state <= 7;
        end
    endcase
end

endmodule
