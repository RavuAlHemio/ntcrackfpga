module ntcrackfpga(
    input clk,
    input [7:0] new_hash_byte,
    input store_hash_byte,
    input go,
    output reg match_found,
    output reg your_turn,
    output reg [7:0] password_byte,
    output reg blinky_led);

`include "gen/inc/encodepwd.v"
`include "src/inc/byteswap.v"
`include "src/inc/md4constants.v"

reg [4:0] state;

reg [159:0] password_chars;
reg [4:0] password_len;
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

reg [127:0] md4_hash_holder;
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
    md4_out_d);

hashchecker hchecker(
    clk,
    checker_newrdy,
    checker_checkrdy,
    md4_hash_holder,
    checker_resultrdy,
    checker_matchfound);

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
    blinky_led <= next_password_chars[0];

    case (state)
        // loading stage
        0: begin
            // wait until we are asked to store a hash byte or to start processing
            if (go) begin
                your_turn <= 0;
                state <= 6;
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
            if (next_hash_byte == 15) begin
                // the hash is complete; store it
                checker_newrdy <= 1;

                next_hash_byte <= 0;
                state <= 3;
            end else begin
                // wait for the next byte
                next_hash_byte <= next_hash_byte + 1;
                your_turn <= 1;

                state <= 0;
            end
        end
        3: begin
            // keep newrdy raised...

            state <= 4;
        end
        4: begin
            // pull newrdy down
            checker_newrdy <= 0;

            state <= 5;
        end
        5: begin
            // wait until the checker has stored the hash, then return back
            if (checker_resultrdy) begin
                your_turn <= 1;
                state <= 0;
            end
        end

        // processing stage
        6: begin
            // load up the current password
            `SET_MD4_INITIAL(md4_in_a, md4_in_b, md4_in_c, md4_in_d);
            md4_data <= password_to_md4_data(password_chars, password_len);

            state <= 7;
        end
        7: begin
            // trigger MD4
            md4_irdy <= 1;

            state <= 8;
        end
        8: begin
            // keep MD4 trigger up

            state <= 9;
        end
        9: begin
            // take down MD4 trigger
            md4_irdy <= 0;

            state <= 10;
        end
        10: begin
            // wait for MD4 to finish
            if (md4_ordy)
                state <= 11;
        end
        11: begin
            // make the MD4 state a hash
            md4_hash_holder <= byteswap_md4(md4_out_a, md4_out_b, md4_out_c, md4_out_d);
            state <= 12;
        end
        12: begin
            // let the hash database think about it
            checker_checkrdy <= 1;
            state <= 13;
        end
        13: begin
            // keep it up for a bit
            state <= 14;
        end
        14: begin
            // pull it down
            checker_checkrdy <= 0;
            state <= 15;
        end
        15: begin
            // wait until it's done
            if (checker_resultrdy)
                state <= 16;
        end
        16: begin
            // have we found anything?
            if (checker_matchfound) begin
                // go into elaboration mode
                password_byte_index <= 0;
                state <= 17;
            end else begin
                // go directly to password incrementing
                state <= 21;
            end
        end

        // elaboration mode
        17: begin
            // prepare a character of the password
            if (password_byte_index == 20) begin
                // send the length as the last byte
                password_byte <= {3'h0, password_len};
            end else begin
                // send that byte of the password
                `MUX_READ_8_OF_160(password_chars, password_byte_index, password_byte);
            end
            state <= 18;
        end
        18: begin
            // raise the "byte is ready" flag
            your_turn <= 1;
            match_found <= 1;
            state <= 19;
        end
        19: begin
            // keep match_found raised until "go"
            if (go) begin
                your_turn <= 0;
                match_found <= 0;
                state <= 20;
            end
        end
        20: begin
            if (password_byte_index == 20) begin
                // this password is done; get on cracking
                state <= 21;
            end else begin
                // go to the next byte in elaboration mode
                password_byte_index <= password_byte_index + 1;
                state <= 17;
            end
        end

        // incrementation
        21: begin
            // increment the password
            increment_password_trigger <= 1;
            state <= 22;
        end
        22: begin
            // keep the flag up
            state <= 23;
        end
        23: begin
            // lower the flag
            increment_password_trigger <= 0;
            state <= 24;
        end
        24: begin
            // wait for incrementor to finish
            if (increment_password_done)
                state <= 25;
        end
        25: begin
            // transfer incremented password and restart processing stage
            password_chars <= next_password_chars;
            password_len <= next_password_len;
            state <= 6;
        end
    endcase
end

endmodule
