module hashchecker (
    input nrst,
    input clk,
    input newrdy,
    input checkrdy,
    input [127:0] hash,
    output reg resultrdy,
    output reg matchfound,

    // debug outputs
    output reg [(64*128-1):0] hashes,
    output reg [3:0] state);

`include "gen/inc/muxes.v"

reg [6:0] current_hash_index;
reg write_trigger;
reg [127:0] current_hash_bits;

always @ (posedge clk) begin
    if (!nrst) begin
        state <= 0;
        current_hash_index <= 0;
        hashes <= 0;
        write_trigger <= 0;
        current_hash_bits <= 0;
        resultrdy <= 0;
        matchfound <= 0;
    end else begin
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
                `MUX_WRITE_128_OF_8192(hashes, current_hash_index, hash);

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
end

endmodule
