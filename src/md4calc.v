module md4calc (
    input clk,
    input irdy,
    input [31:0] state_a,
    input [31:0] state_b,
    input [31:0] state_c,
    input [31:0] state_d,
    input [511:0] data,
    output reg ordy,
    output reg [31:0] newstate_a,
    output reg [31:0] newstate_b,
    output reg [31:0] newstate_c,
    output reg [31:0] newstate_d);

`include "src/inc/byteswap.v"

reg step;
reg [31:0] aa;
reg [31:0] bb;
reg [31:0] cc;
reg [31:0] dd;

function [31:0] md4_f (
    input [31:0] x,
    input [31:0] y,
    input [31:0] z);
    begin
        md4_f = (x & y) | ((~x) & z);
    end
endfunction

function [31:0] md4_g (
    input [31:0] x,
    input [31:0] y,
    input [31:0] z);
    begin
        md4_g = (x & y) | (x & z) | (y & z);
    end
endfunction

function [31:0] md4_h (
    input [31:0] x,
    input [31:0] y,
    input [31:0] z);
    begin
        md4_h = x ^ y ^ z;
    end
endfunction

`define DEFINE_ROTATE_LEFT(name, bits, shift) function [bits-1:0] name (input [bits-1:0] in); begin name = { in[bits-(1+shift):0], in[bits-1:bits-shift] }; end endfunction
`DEFINE_ROTATE_LEFT(rol3, 32, 3)
`DEFINE_ROTATE_LEFT(rol5, 32, 5)
`DEFINE_ROTATE_LEFT(rol7, 32, 7)
`DEFINE_ROTATE_LEFT(rol9, 32, 9)
`DEFINE_ROTATE_LEFT(rol11, 32, 11)
`DEFINE_ROTATE_LEFT(rol13, 32, 13)
`DEFINE_ROTATE_LEFT(rol15, 32, 15)
`DEFINE_ROTATE_LEFT(rol19, 32, 19)

`define OP_F(k, l, m, n, dataoffset, rotfunc) k <= rotfunc(k + md4_f(l, m, n) + byteswap32(data[511-(dataoffset*32):511-(dataoffset*32+31)]))
`define OP_F_CASES(i0, i1, i2, i3, off0, off1, off2, off3) \
    i0: `OP_F(aa, bb, cc, dd, off0, rol3); \
    i1: `OP_F(dd, aa, bb, cc, off1, rol7); \
    i2: `OP_F(cc, dd, aa, bb, off2, rol11); \
    i3: `OP_F(bb, cc, dd, aa, off3, rol19);

`define OP_G(k, l, m, n, dataoffset, rotfunc) #1 k <= rotfunc(k + md4_g(l, m, n) + byteswap32(data[511-(dataoffset*32):511-(dataoffset*32+31)]) + 32'h5A827999)
`define OP_G_BLOCK(off0, off1, off2, off3) \
    `OP_G(aa, bb, cc, dd, off0, rol3); \
    `OP_G(dd, aa, bb, cc, off1, rol5); \
    `OP_G(cc, dd, aa, bb, off2, rol9); \
    `OP_G(bb, cc, dd, aa, off3, rol13)

`define OP_H(k, l, m, n, dataoffset, rotfunc) #1 k <= rotfunc(k + md4_h(l, m, n) + byteswap32(data[511-(dataoffset*32):511-(dataoffset*32+31)]) + 32'h6ED9EBA1)
`define OP_H_BLOCK(off0, off1, off2, off3) \
    `OP_H(aa, bb, cc, dd, off0, rol3); \
    `OP_H(dd, aa, bb, cc, off1, rol9); \
    `OP_H(cc, dd, aa, bb, off2, rol11); \
    `OP_H(bb, cc, dd, aa, off3, rol15)

always @ (posedge clk) begin
    case (step)
        0: begin
            aa <= state_a;
            bb <= state_b;
            cc <= state_c;
            dd <= state_d;
        end
        //          | step values | offsets in op |
        `OP_F_CASES( 1,  2,  3,  4,  0,  1,  2,  3);
        `OP_F_CASES( 5,  6,  7,  8,  4,  5,  6,  7);
        `OP_F_CASES( 9, 10, 11, 12,  8,  9, 10, 11);
        `OP_F_CASES(13, 14, 15, 16, 12, 13, 14, 15);
    endcase
end

always @ (posedge irdy) begin
    #1
    aa <= state_a;
    bb <= state_b;
    cc <= state_c;
    dd <= state_d;

    `OP_G_BLOCK( 0,  4,  8, 12);
    `OP_G_BLOCK( 1,  5,  9, 13);
    `OP_G_BLOCK( 2,  6, 10, 14);
    `OP_G_BLOCK( 3,  7, 11, 15);

    `OP_H_BLOCK( 0,  8,  4, 12);
    `OP_H_BLOCK( 2, 10,  6, 14);
    `OP_H_BLOCK( 1,  9,  5, 13);
    `OP_H_BLOCK( 3, 11,  7, 15);

    #1
    newstate_a <= state_a + aa;
    newstate_b <= state_b + bb;
    newstate_c <= state_c + cc;
    newstate_d <= state_d + dd;

    #1
    ordy <= 1;

    #1
    ordy <= 0;
end

endmodule
