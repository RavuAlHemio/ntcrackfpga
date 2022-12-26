module state_giver(
    input clk,
    input [4:0] password_len,
    input [159:0] password_chars,
    input [(128*128-1):0] hashes,
    input [127:0] current_hash,
    output reg [7:0] state_byte);

reg [11:0] byte_index;

initial begin
    byte_index <= 0;
end

always @ (posedge clk) begin
    case (byte_index)
        /* header */
        0: state_byte <= 8'h0A;
        1: state_byte <= 8'h55;
        2: state_byte <= 8'hFA;
        3: state_byte <= 8'hCE;

        // password length
        4: state_byte <= password_len;

        // password characters
        5: state_byte <= password_chars[159:152];
        6: state_byte <= password_chars[151:144];
        7: state_byte <= password_chars[143:136];
        8: state_byte <= password_chars[135:128];
        9: state_byte <= password_chars[127:120];
        10: state_byte <= password_chars[119:112];
        11: state_byte <= password_chars[111:104];
        12: state_byte <= password_chars[103:96];
        13: state_byte <= password_chars[95:88];
        14: state_byte <= password_chars[87:80];
        15: state_byte <= password_chars[79:72];
        16: state_byte <= password_chars[71:64];
        17: state_byte <= password_chars[63:56];
        18: state_byte <= password_chars[55:48];
        19: state_byte <= password_chars[47:40];
        20: state_byte <= password_chars[39:32];
        21: state_byte <= password_chars[31:24];
        22: state_byte <= password_chars[23:16];
        23: state_byte <= password_chars[15:8];
        24: state_byte <= password_chars[7:0];

        // stored hashes
        // Python:
        // for n in range(2048):
        //     if n % 16 == 0:
        //         print()
        //     print(f"        {n+25}: state_byte <= hashes[{128*128-(8*n+1)}:{128*128-(8*n+8)}];")
        25: state_byte <= hashes[16383:16376];
        26: state_byte <= hashes[16375:16368];
        27: state_byte <= hashes[16367:16360];
        28: state_byte <= hashes[16359:16352];
        29: state_byte <= hashes[16351:16344];
        30: state_byte <= hashes[16343:16336];
        31: state_byte <= hashes[16335:16328];
        32: state_byte <= hashes[16327:16320];
        33: state_byte <= hashes[16319:16312];
        34: state_byte <= hashes[16311:16304];
        35: state_byte <= hashes[16303:16296];
        36: state_byte <= hashes[16295:16288];
        37: state_byte <= hashes[16287:16280];
        38: state_byte <= hashes[16279:16272];
        39: state_byte <= hashes[16271:16264];
        40: state_byte <= hashes[16263:16256];

        41: state_byte <= hashes[16255:16248];
        42: state_byte <= hashes[16247:16240];
        43: state_byte <= hashes[16239:16232];
        44: state_byte <= hashes[16231:16224];
        45: state_byte <= hashes[16223:16216];
        46: state_byte <= hashes[16215:16208];
        47: state_byte <= hashes[16207:16200];
        48: state_byte <= hashes[16199:16192];
        49: state_byte <= hashes[16191:16184];
        50: state_byte <= hashes[16183:16176];
        51: state_byte <= hashes[16175:16168];
        52: state_byte <= hashes[16167:16160];
        53: state_byte <= hashes[16159:16152];
        54: state_byte <= hashes[16151:16144];
        55: state_byte <= hashes[16143:16136];
        56: state_byte <= hashes[16135:16128];

        // current hash
        57: state_byte <= current_hash[127:120];
        58: state_byte <= current_hash[119:112];
        59: state_byte <= current_hash[111:104];
        60: state_byte <= current_hash[103:96];
        61: state_byte <= current_hash[95:88];
        62: state_byte <= current_hash[87:80];
        63: state_byte <= current_hash[79:72];
        64: state_byte <= current_hash[71:64];
        65: state_byte <= current_hash[63:56];
        66: state_byte <= current_hash[55:48];
        67: state_byte <= current_hash[47:40];
        68: state_byte <= current_hash[39:32];
        69: state_byte <= current_hash[31:24];
        70: state_byte <= current_hash[23:16];
        71: state_byte <= current_hash[15:8];
        72: state_byte <= current_hash[7:0];

        // footer
        73: state_byte <= 8'hA2;
        74: state_byte <= 8'h5E;
        75: state_byte <= 8'hFA;
        76: state_byte <= 8'hCE;

    endcase

    if (byte_index == 76)
        byte_index <= 0;
    else
        byte_index <= byte_index + 1;
end

endmodule
