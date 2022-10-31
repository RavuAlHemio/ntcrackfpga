function [31:0] byteswap32 (
    input [31:0] b);
    begin
        byteswap32 = {b[7:0], b[15:8], b[23:16], b[31:24]};
    end
endfunction

function [127:0] byteswap_md4 (
    input [31:0] md4_a,
    input [31:0] md4_b,
    input [31:0] md4_c,
    input [31:0] md4_d);
    begin
        byteswap_md4 = {
            md4_a[7:0], md4_a[15:8], md4_a[23:16], md4_a[31:24],
            md4_b[7:0], md4_b[15:8], md4_b[23:16], md4_b[31:24],
            md4_c[7:0], md4_c[15:8], md4_c[23:16], md4_c[31:24],
            md4_d[7:0], md4_d[15:8], md4_d[23:16], md4_d[31:24]};
    end
endfunction
