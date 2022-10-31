module md4pwgen(
    input clk,
    input start_flag,
    input checker_resultrdy,
    input checker_matchfound,
    output reg [127:0] checker_next_hash,
    output reg checker_checkrdy);

reg state;
reg [159:0] current_password;
reg [5:0] current_password_len;
reg [7:0] current_password_last_byte;

reg md4_irdy;
reg [31:0] md4_state_a;
reg [31:0] md4_state_b;
reg [31:0] md4_state_c;
reg [31:0] md4_state_d;
reg [511:0] md4_data;
reg md4_ordy;
reg [31:0] md4_newstate_a;
reg [31:0] md4_newstate_b;
reg [31:0] md4_newstate_c;
reg [31:0] md4_newstate_d;

reg write_mux_trigger;
reg read_mux_trigger;


md4block md4calc(
    clk,
    md4_irdy,
    md4_state_a,
    md4_state_b,
    md4_state_c,
    md4_state_d,
    md4_data,
    md4_ordy,
    md4_newstate_a,
    md4_newstate_b,
    md4_newstate_c,
    md4_newstate_d);


mux_write_8_of_160 password_write_mux(
    current_password,
    current_password_len,
    write_mux_trigger,
    current_password_last_byte);

mux_read_8_of_160 password_read_mux(
    current_password,
    current_password_len,
    read_mux_trigger,
    current_password_last_byte);

initial begin
    state <= 0;
    current_password <= 0;
    current_password_len <= 0;
end

always @ (posedge clk) begin
    case (state)
        0: begin
            // awaiting start flag
            if (start_flag)
                state <= 1;
        end
        1: begin
            // initialize first password
            current_password <= 0;
            current_password_len <= 0;

            state <= 2;
        end
        2: begin
            // initialize MD4 state
            md4_in_a <= 32'h67452301;
            md4_in_b <= 32'hefcdab89;
            md4_in_c <= 32'h98badcfe;
            md4_in_d <= 32'h10325476;
            md4_data <= password_to_md4_data(current_password, current_password_len);

            state <= 3;
        end
        3: begin
            // trigger MD4
            md4_irdy <= 1;

            state <= 4;
        end
        4: begin
            // un-trigger MD4
            md4_irdy <= 0;

            state <= 5;
        end
        5: begin
            // wait for MD4 to finish
            if (md4_ordy)
                state <= 6;
        end
        6: begin
            // transform hash
            checker_next_hash <= byteswap_md4(md4_out_a, md4_out_b, md4_out_c, md4_out_d);

            state <= 7;
        end
        7: begin
            // trigger checker
            checker_checkrdy <= 1;

            state <= 8;
        end
        8: begin
            // untrigger checker
            checker_checkrdy <= 0;

            state <= 9;
        end
        9: begin
            // wait for checker to finish
            if (checker_resultrdy)
                state <= 10;
        end
        10: begin
            if (checker_matchfound) begin
                // TODO: output successful password somehow...
            end
            state <= 11;
        end
        11: begin
            // TODO: wait for password output to finish...
            state <= 12;
        end
        12: begin
            // advance password
            if (current_password_len == 0) begin
                // first printable character is 0x20 (space)
                current_password <= {160'h2020202020202020202020202020202020202020};
                current_password_len <= 1;

                // continue with MD4
                state <= 2;
            end else begin
                // trigger demuxer
                read_mux_trigger <= 1;
                state <= 13;
            end
        end
        13: begin
            // untrigger demuxer
            read_mux_trigger <= 0;

            state <= 14;
        end
        14: begin
            // last printable character is 0x7E (tilde)
            if (current_password_last_byte == 8'h7E) begin
                // increase 
            end
        end
    endcase
end



endmodule
