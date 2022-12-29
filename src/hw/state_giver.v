module state_giver(
    input nrst,
    input clk,
    input [4:0] password_len,
    input [159:0] password_chars,
    input [(128*128-1):0] hashes,
    input [127:0] current_hash,
    input [4:0] ntcrackfpga_state,
    input [3:0] hashchecker_state,
    input [5:0] md4block_step,
    output reg [7:0] state_byte);

reg [11:0] byte_index;

always @ (posedge clk) begin
    if (!nrst) begin
        byte_index <= 0;
    end else begin
        case (byte_index)
            // header
            0: state_byte <= 8'h0A;
            1: state_byte <= 8'h55;
            2: state_byte <= 8'hFA;
            3: state_byte <= 8'hCE;
            4: state_byte <= 8'h01;

            // 1 byte follows

            // password length
            5: state_byte <= {3'h0, password_len};

            // header
            6: state_byte <= 8'h0A;
            7: state_byte <= 8'h55;
            8: state_byte <= 8'hFA;
            9: state_byte <= 8'hCE;
            10: state_byte <= 8'h02;

            // 20 bytes follow

            // password characters
            11: state_byte <= password_chars[159:152];
            12: state_byte <= password_chars[151:144];
            13: state_byte <= password_chars[143:136];
            14: state_byte <= password_chars[135:128];
            15: state_byte <= password_chars[127:120];
            16: state_byte <= password_chars[119:112];
            17: state_byte <= password_chars[111:104];
            18: state_byte <= password_chars[103:96];
            19: state_byte <= password_chars[95:88];
            20: state_byte <= password_chars[87:80];
            21: state_byte <= password_chars[79:72];
            22: state_byte <= password_chars[71:64];
            23: state_byte <= password_chars[63:56];
            24: state_byte <= password_chars[55:48];
            25: state_byte <= password_chars[47:40];
            26: state_byte <= password_chars[39:32];
            27: state_byte <= password_chars[31:24];
            28: state_byte <= password_chars[23:16];
            29: state_byte <= password_chars[15:8];
            30: state_byte <= password_chars[7:0];

            // header
            31: state_byte <= 8'h0A;
            32: state_byte <= 8'h55;
            33: state_byte <= 8'hFA;
            34: state_byte <= 8'hCE;
            35: state_byte <= 8'h03;

            // 32 bytes follow

            // stored hashes
            // Python:
            // for n in range(2048):
            //     if n % 16 == 0:
            //         print()
            //     print(f"        {n+36}: state_byte <= hashes[{128*128-(8*n+1)}:{128*128-(8*n+8)}];")
            36: state_byte <= hashes[16383:16376];
            37: state_byte <= hashes[16375:16368];
            38: state_byte <= hashes[16367:16360];
            39: state_byte <= hashes[16359:16352];
            40: state_byte <= hashes[16351:16344];
            41: state_byte <= hashes[16343:16336];
            42: state_byte <= hashes[16335:16328];
            43: state_byte <= hashes[16327:16320];
            44: state_byte <= hashes[16319:16312];
            45: state_byte <= hashes[16311:16304];
            46: state_byte <= hashes[16303:16296];
            47: state_byte <= hashes[16295:16288];
            48: state_byte <= hashes[16287:16280];
            49: state_byte <= hashes[16279:16272];
            50: state_byte <= hashes[16271:16264];
            51: state_byte <= hashes[16263:16256];

            52: state_byte <= hashes[16255:16248];
            53: state_byte <= hashes[16247:16240];
            54: state_byte <= hashes[16239:16232];
            55: state_byte <= hashes[16231:16224];
            56: state_byte <= hashes[16223:16216];
            57: state_byte <= hashes[16215:16208];
            58: state_byte <= hashes[16207:16200];
            59: state_byte <= hashes[16199:16192];
            60: state_byte <= hashes[16191:16184];
            61: state_byte <= hashes[16183:16176];
            62: state_byte <= hashes[16175:16168];
            63: state_byte <= hashes[16167:16160];
            64: state_byte <= hashes[16159:16152];
            65: state_byte <= hashes[16151:16144];
            66: state_byte <= hashes[16143:16136];
            67: state_byte <= hashes[16135:16128];

            // header
            68: state_byte <= 8'h0A;
            69: state_byte <= 8'h55;
            70: state_byte <= 8'hFA;
            71: state_byte <= 8'hCE;
            72: state_byte <= 8'h04;

            // 16 bytes follow

            // current hash
            73: state_byte <= current_hash[127:120];
            74: state_byte <= current_hash[119:112];
            75: state_byte <= current_hash[111:104];
            76: state_byte <= current_hash[103:96];
            77: state_byte <= current_hash[95:88];
            78: state_byte <= current_hash[87:80];
            79: state_byte <= current_hash[79:72];
            80: state_byte <= current_hash[71:64];
            81: state_byte <= current_hash[63:56];
            82: state_byte <= current_hash[55:48];
            83: state_byte <= current_hash[47:40];
            84: state_byte <= current_hash[39:32];
            85: state_byte <= current_hash[31:24];
            86: state_byte <= current_hash[23:16];
            87: state_byte <= current_hash[15:8];
            88: state_byte <= current_hash[7:0];

            // header
            89: state_byte <= 8'h0A;
            90: state_byte <= 8'h55;
            91: state_byte <= 8'hFA;
            92: state_byte <= 8'hCE;
            93: state_byte <= 8'h05;

            // 3 bytes follow

            // state counters
            94: state_byte <= {3'h0, ntcrackfpga_state};
            95: state_byte <= {4'h0, hashchecker_state};
            96: state_byte <= {2'h0, md4block_step};

            // footer
            97: state_byte <= 8'hA2;
            98: state_byte <= 8'h5E;
            99: state_byte <= 8'hFA;
            100: state_byte <= 8'hCE;

        endcase

        if (byte_index == 100)
            byte_index <= 0;
        else
            byte_index <= byte_index + 1;
    end
end

endmodule
