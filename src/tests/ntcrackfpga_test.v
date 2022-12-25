module ntcrackfpga_test();

`include "gen/inc/muxes.v"

reg clk;
reg [7:0] new_hash_byte;
reg store_hash_byte;
reg go;
wire match_found;
wire my_turn;
wire [7:0] password_byte;
wire blinky_led;

reg [7:0] state;
reg [167:0] password;

ntcrackfpga cracker(
    clk,
    new_hash_byte,
    store_hash_byte,
    go,
    match_found,
    my_turn,
    password_byte,
    blinky_led);

initial begin
    $dumpfile("ntcrackfpga_test.vcd");
    $dumpvars(0, ntcrackfpga_test);

    clk <= 0;
    new_hash_byte <= 0;
    store_hash_byte <= 0;
    go <= 0;

    state <= 0;
    found_password_count <= 0;
end

always begin
    #1 clk <= !clk;
end

`define STORE_HASH_BYTE(start_time, hash, bottom_bit) \
    (start_time+0): begin \
        if (my_turn) begin \
            state <= start_time+1; \
        end \
    end \
    (start_time+1): begin \
        new_hash_byte <= (((hash) >> bottom_bit) & 8'hFF); \
        state <= start_time+2; \
    end \
    (start_time+2): begin \
        store_hash_byte <= 1; \
        state <= start_time+3; \
    end \
    (start_time+3): begin \
        state <= start_time+4; \
    end \
    (start_time+4): begin \
        store_hash_byte <= 0; \
        state <= start_time+5; \
    end

`define STORE_HASH(initial_time, hash) \
    `STORE_HASH_BYTE(initial_time+0, hash, 0) \
    `STORE_HASH_BYTE(initial_time+5, hash, 8) \
    `STORE_HASH_BYTE(initial_time+10, hash, 16) \
    `STORE_HASH_BYTE(initial_time+15, hash, 24) \
    `STORE_HASH_BYTE(initial_time+20, hash, 32) \
    `STORE_HASH_BYTE(initial_time+25, hash, 40) \
    `STORE_HASH_BYTE(initial_time+30, hash, 48) \
    `STORE_HASH_BYTE(initial_time+35, hash, 56) \
    `STORE_HASH_BYTE(initial_time+40, hash, 64) \
    `STORE_HASH_BYTE(initial_time+45, hash, 72) \
    `STORE_HASH_BYTE(initial_time+50, hash, 80) \
    `STORE_HASH_BYTE(initial_time+55, hash, 88) \
    `STORE_HASH_BYTE(initial_time+60, hash, 96) \
    `STORE_HASH_BYTE(initial_time+65, hash, 104) \
    `STORE_HASH_BYTE(initial_time+70, hash, 112) \
    `STORE_HASH_BYTE(initial_time+75, hash, 120)
    // next step at initial_time + 80

`define STORE_PASSWORD_BYTE(initial_time, bottom_bit) \
    (initial_time+0): begin \
        if (my_turn) begin \
            password[bottom_bit+7:bottom_bit] <= password_byte; \
            state <= initial_time+1; \
        end \
    end \
    (initial_time+1): begin \
        go <= 1; \
        state <= initial_time+2; \
    end \
    (initial_time+2): begin \
        state <= initial_time+3; \
    end \
    (initial_time+3): begin \
        go <= 0; \
        state <= initial_time+4; \
    end
    // next step at initial_time + 4


always @ (posedge clk) begin
    // populate the following hashes:
    // md4(utf16le("12")) == md4("\x31\x00\x32\x00") == 588FEB889288FB953B5F094D47D1565C
    // md4(utf16le("!?")) == md4("\x21\x00\x3F\x00") == 91D533DC611AC2774431E2D0BAF36805
    case (state)
        `STORE_HASH(0, 128'h588FEB889288FB953B5F094D47D1565C)
        `STORE_HASH(80, 128'h91D533DC611AC2774431E2D0BAF36805)

        160: begin
            if (my_turn) begin
                state <= 161;
            end
        end
        161: begin
            $display("%x", cracker.hchecker.hashes);
            $display("%d", cracker.hchecker.current_hash_index);
            go <= 1;
            state <= 162;
        end
        162: begin
            state <= 163;
        end
        163: begin
            go <= 0;
            state <= 164;
        end
        164: begin
            // wait for it
            if (my_turn) begin
                if (match_found) begin
                    // we have a password
                    state <= 165;
                end else begin
                    // we are done
                    $finish;
                end
            end
        end

        // read the finished password
        `STORE_PASSWORD_BYTE(165, 152)
        `STORE_PASSWORD_BYTE(169, 144)
        `STORE_PASSWORD_BYTE(173, 136)
        `STORE_PASSWORD_BYTE(177, 128)
        `STORE_PASSWORD_BYTE(181, 120)
        `STORE_PASSWORD_BYTE(185, 112)
        `STORE_PASSWORD_BYTE(189, 104)
        `STORE_PASSWORD_BYTE(193, 96)
        `STORE_PASSWORD_BYTE(197, 88)
        `STORE_PASSWORD_BYTE(201, 80)
        `STORE_PASSWORD_BYTE(205, 72)
        `STORE_PASSWORD_BYTE(209, 64)
        `STORE_PASSWORD_BYTE(213, 56)
        `STORE_PASSWORD_BYTE(217, 48)
        `STORE_PASSWORD_BYTE(221, 40)
        `STORE_PASSWORD_BYTE(225, 32)
        `STORE_PASSWORD_BYTE(229, 24)
        `STORE_PASSWORD_BYTE(233, 16)
        `STORE_PASSWORD_BYTE(237, 8)
        `STORE_PASSWORD_BYTE(241, 0)

        // and the length
        `STORE_PASSWORD_BYTE(245, 160)

        249: begin
            // spit out password
            $display("found: %x", password);

            // wait for next event
            state <= 164;
        end
    endcase
end

endmodule
