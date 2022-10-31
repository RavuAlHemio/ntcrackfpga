module hashchecker (
    input clk,
    input newrdy,
    input checkrdy,
    input [127:0] hash,
    output reg resultrdy,
    output reg matchfound);

`include "gen/inc/muxes.v"

reg [3:0] state;
reg [6:0] current_hash_index;
reg [(128*128-1):0] hashes;
reg write_trigger;
reg [127:0] current_hash_bits;

initial begin
    state <= 0;
    current_hash_index <= 0;
    hashes <= 0;
    write_trigger <= 0;
    current_hash_bits <= 0;
    resultrdy <= 0;
    matchfound <= 0;
end

always @ (posedge clk) begin
    case (state)
        0: begin
            // wait for one of the signals
            if (newrdy)
                state <= 1;
            else if (checkrdy)
                state <= 5;
        end

        // "new" logic
        1: begin
            // store
            `MUX_WRITE_128_OF_16384(hashes, current_hash_index, hash);

            state <= 2;
        end
        2: begin
            // increment current hash index
            current_hash_index <= current_hash_index + 1;

            state <= 3;
        end
        3: begin
            // inform that we are done
            resultrdy <= 1;

            state <= 4;
        end
        4: begin
            // remove inform flag
            resultrdy <= 0;

            // return to dormant state
            state <= 0;
        end

        // "check" logic
        5: begin
            // set default result
            matchfound <= 0;

            state <= 6;
        end
        6: begin
            // check it all
            `define HASH_MATCHES(i) if (hashes[((i+1)*128-1):(i*128)] == hash) begin matchfound <= 1; end
            `HASH_MATCHES(0)
            `HASH_MATCHES(1)
            `HASH_MATCHES(2)
            `HASH_MATCHES(3)
            `HASH_MATCHES(4)
            `HASH_MATCHES(5)
            `HASH_MATCHES(6)
            `HASH_MATCHES(7)
            `HASH_MATCHES(8)
            `HASH_MATCHES(9)
            `HASH_MATCHES(10)
            `HASH_MATCHES(11)
            `HASH_MATCHES(12)
            `HASH_MATCHES(13)
            `HASH_MATCHES(14)
            `HASH_MATCHES(15)
            `HASH_MATCHES(16)
            `HASH_MATCHES(17)
            `HASH_MATCHES(18)
            `HASH_MATCHES(19)
            `HASH_MATCHES(20)
            `HASH_MATCHES(21)
            `HASH_MATCHES(22)
            `HASH_MATCHES(23)
            `HASH_MATCHES(24)
            `HASH_MATCHES(25)
            `HASH_MATCHES(26)
            `HASH_MATCHES(27)
            `HASH_MATCHES(28)
            `HASH_MATCHES(29)
            `HASH_MATCHES(30)
            `HASH_MATCHES(31)
            `HASH_MATCHES(32)
            `HASH_MATCHES(33)
            `HASH_MATCHES(34)
            `HASH_MATCHES(35)
            `HASH_MATCHES(36)
            `HASH_MATCHES(37)
            `HASH_MATCHES(38)
            `HASH_MATCHES(39)
            `HASH_MATCHES(40)
            `HASH_MATCHES(41)
            `HASH_MATCHES(42)
            `HASH_MATCHES(43)
            `HASH_MATCHES(44)
            `HASH_MATCHES(45)
            `HASH_MATCHES(46)
            `HASH_MATCHES(47)
            `HASH_MATCHES(48)
            `HASH_MATCHES(49)
            `HASH_MATCHES(50)
            `HASH_MATCHES(51)
            `HASH_MATCHES(52)
            `HASH_MATCHES(53)
            `HASH_MATCHES(54)
            `HASH_MATCHES(55)
            `HASH_MATCHES(56)
            `HASH_MATCHES(57)
            `HASH_MATCHES(58)
            `HASH_MATCHES(59)
            `HASH_MATCHES(60)
            `HASH_MATCHES(61)
            `HASH_MATCHES(62)
            `HASH_MATCHES(63)
            `HASH_MATCHES(64)
            `HASH_MATCHES(65)
            `HASH_MATCHES(66)
            `HASH_MATCHES(67)
            `HASH_MATCHES(68)
            `HASH_MATCHES(69)
            `HASH_MATCHES(70)
            `HASH_MATCHES(71)
            `HASH_MATCHES(72)
            `HASH_MATCHES(73)
            `HASH_MATCHES(74)
            `HASH_MATCHES(75)
            `HASH_MATCHES(76)
            `HASH_MATCHES(77)
            `HASH_MATCHES(78)
            `HASH_MATCHES(79)
            `HASH_MATCHES(80)
            `HASH_MATCHES(81)
            `HASH_MATCHES(82)
            `HASH_MATCHES(83)
            `HASH_MATCHES(84)
            `HASH_MATCHES(85)
            `HASH_MATCHES(86)
            `HASH_MATCHES(87)
            `HASH_MATCHES(88)
            `HASH_MATCHES(89)
            `HASH_MATCHES(90)
            `HASH_MATCHES(91)
            `HASH_MATCHES(92)
            `HASH_MATCHES(93)
            `HASH_MATCHES(94)
            `HASH_MATCHES(95)
            `HASH_MATCHES(96)
            `HASH_MATCHES(97)
            `HASH_MATCHES(98)
            `HASH_MATCHES(99)
            `HASH_MATCHES(100)
            `HASH_MATCHES(101)
            `HASH_MATCHES(102)
            `HASH_MATCHES(103)
            `HASH_MATCHES(104)
            `HASH_MATCHES(105)
            `HASH_MATCHES(106)
            `HASH_MATCHES(107)
            `HASH_MATCHES(108)
            `HASH_MATCHES(109)
            `HASH_MATCHES(110)
            `HASH_MATCHES(111)
            `HASH_MATCHES(112)
            `HASH_MATCHES(113)
            `HASH_MATCHES(114)
            `HASH_MATCHES(115)
            `HASH_MATCHES(116)
            `HASH_MATCHES(117)
            `HASH_MATCHES(118)
            `HASH_MATCHES(119)
            `HASH_MATCHES(120)
            `HASH_MATCHES(121)
            `HASH_MATCHES(122)
            `HASH_MATCHES(123)
            `HASH_MATCHES(124)
            `HASH_MATCHES(125)
            `HASH_MATCHES(126)
            `HASH_MATCHES(127)

            state <= 7;
        end
        7: begin
            // inform that we are done
            resultrdy <= 1;

            state <= 8;
        end
        8: begin
            // leave the flag up for a bit

            state <= 9;
        end
        9: begin
            // lower the flag again
            resultrdy <= 0;

            // go dormant
            state <= 0;
        end
    endcase
end

endmodule
