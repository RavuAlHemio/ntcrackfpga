module md4block (
    input nrst,
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
    output reg [31:0] newstate_d,

    // debug outputs
    output reg [5:0] step);

`include "src/inc/byteswap.v"

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
    i3: `OP_F(bb, cc, dd, aa, off3, rol19)

`define OP_G(k, l, m, n, dataoffset, rotfunc) k <= rotfunc(k + md4_g(l, m, n) + byteswap32(data[511-(dataoffset*32):511-(dataoffset*32+31)]) + 32'h5A827999)
`define OP_G_CASES(i0, i1, i2, i3, off0, off1, off2, off3) \
    i0: `OP_G(aa, bb, cc, dd, off0, rol3); \
    i1: `OP_G(dd, aa, bb, cc, off1, rol5); \
    i2: `OP_G(cc, dd, aa, bb, off2, rol9); \
    i3: `OP_G(bb, cc, dd, aa, off3, rol13)

`define OP_H(k, l, m, n, dataoffset, rotfunc) k <= rotfunc(k + md4_h(l, m, n) + byteswap32(data[511-(dataoffset*32):511-(dataoffset*32+31)]) + 32'h6ED9EBA1)
`define OP_H_CASES(i0, i1, i2, i3, off0, off1, off2, off3) \
    i0: `OP_H(aa, bb, cc, dd, off0, rol3); \
    i1: `OP_H(dd, aa, bb, cc, off1, rol9); \
    i2: `OP_H(cc, dd, aa, bb, off2, rol11); \
    i3: `OP_H(bb, cc, dd, aa, off3, rol15)


always @ (posedge clk) begin
    if (!nrst) begin
        step <= 0;
    end else begin
        if (step == 0) begin
            // wait for irdy to be raised
            if (irdy)
                step <= 1;
        end else if (step == 1) begin
            // wait for irdy to be lowered again
            if (!irdy)
                step <= 2;
        end else begin
            // do the calculation step
            case (step)
                2: begin
                    aa <= state_a;
                    bb <= state_b;
                    cc <= state_c;
                    dd <= state_d;
                end

                //          | step values | offsets in op |

                `OP_F_CASES( 3,  4,  5,  6,  0,  1,  2,  3);
                `OP_F_CASES( 7,  8,  9, 10,  4,  5,  6,  7);
                `OP_F_CASES(11, 12, 13, 14,  8,  9, 10, 11);
                `OP_F_CASES(15, 16, 17, 18, 12, 13, 14, 15);

                `OP_G_CASES(19, 20, 21, 22,  0,  4,  8, 12);
                `OP_G_CASES(23, 24, 25, 26,  1,  5,  9, 13);
                `OP_G_CASES(27, 28, 29, 30,  2,  6, 10, 14);
                `OP_G_CASES(31, 32, 33, 34,  3,  7, 11, 15);

                `OP_H_CASES(35, 36, 37, 38,  0,  8,  4, 12);
                `OP_H_CASES(39, 40, 41, 42,  2, 10,  6, 14);
                `OP_H_CASES(43, 44, 45, 46,  1,  9,  5, 13);
                `OP_H_CASES(47, 48, 49, 50,  3, 11,  7, 15);

                51: begin
                    newstate_a <= state_a + aa;
                    newstate_b <= state_b + bb;
                    newstate_c <= state_c + cc;
                    newstate_d <= state_d + dd;
                end
                52: ordy <= 1;
                53: /* keep ordy up */ ;
                54: ordy <= 0;
            endcase

            // advance the state machine
            if (step == 54)
                step <= 0;
            else
                step <= step + 1;
        end
    end
end

endmodule
