module md4test(
    input clk,
    input [7:0] new_hash_byte,
    input store_hash_byte,
    input run_md4,
    output reg is_done,
    output reg match_found);

`include "gen/inc/encodepwd.v"
`include "src/inc/byteswap.v"

reg [159:0] passwd_chars;
reg [7:0] passwd_len;

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

reg write_next_hash;
reg [2:0] next_hash;

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
    checker_newrdy,
    checker_checkrdy,
    md4_hash_holder,
    checker_resultrdy,
    checker_matchfound);

initial begin
    // dump!
    $dumpfile("md4test.vcd");
    $dumpvars(0, md4test);

    really_checking <= 0;
    write_next_hash <= 0;
    match_found <= 0;
    is_done <= 0;

    next_hash <= 0;

    // start loading hashes
    #1
    write_next_hash <= 1;

    #1
    write_next_hash <= 0;
end

always @ (posedge store_hash_byte) begin
    md4_hash_holder <= new_hash_byte;

    #1
    checker_newrdy <= 1;

    #1
    checker_newrdy <= 0;

    #1
    next_hash <= next_hash + 1;
end

always @ (posedge run_md4) begin
    // only start reacting to checker_resultrdy now
    #1
    really_checking <= 1;

    // run the algo
    passwd_chars <= {32'h66617274, 128'h00};
    passwd_len <= 4;

    #1
    md4_in_a <= 32'h67452301;
    md4_in_b <= 32'hefcdab89;
    md4_in_c <= 32'h98badcfe;
    md4_in_d <= 32'h10325476;
    md4_data <= password_to_md4_data(passwd_chars, passwd_len);

    #1
    md4_irdy <= 1;

    #1
    md4_irdy <= 0;
end

always @ (posedge md4_ordy) begin
    /*
    $display("%08x %08x %08x %08x", out_a, out_b, out_c, out_d);
    $display("%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x",
        out_a[7:0], out_a[15:8], out_a[23:16], out_a[31:24],
        out_b[7:0], out_b[15:8], out_b[23:16], out_b[31:24],
        out_c[7:0], out_c[15:8], out_c[23:16], out_c[31:24],
        out_d[7:0], out_d[15:8], out_d[23:16], out_d[31:24]);
    $finish;
    */

    // run the checker
    md4_hash_holder <= byteswap_md4(md4_out_a, md4_out_b, md4_out_c, md4_out_d);
    $display("%032x", md4_hash_holder);

    #1
    checker_checkrdy <= 1;

    #1
    checker_checkrdy <= 0;
end

always @ (posedge checker_resultrdy && really_checking == 1) begin
    match_found <= checker_matchfound;
    is_done <= 1;
end

endmodule
