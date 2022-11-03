# ntcrackfpga Protocol

This documents the communication protocol between the cracker (running on an FPGA) and a driver (most probably a microcontroller).

| direction        | usage                 |
| ---------------- | --------------------- |
| driver → cracker | `store_hash_byte`     |
| driver → cracker | `go`                  |
| driver ← cracker | `match_found`         |
| driver ← cracker | `your_turn`           |
| driver → cracker | `new_hash_byte[0..7]` |
| driver ← cracker | `password_byte[0..7]` |

The following protocol is expected:

1. The driver awaits HIGH on `your_turn`.

2. The driver places a byte of one of the hashes to match on `new_hash_byte[0..7]`. Hashes are to be transmitted byte-for-byte with the most significant byte first. Each MD4 hash consists of 16 bytes.

3. The driver raises and lowers `store_hash_byte`.

4. Steps 1 through 3 are repeated for all hashes to transmit.

5. The driver raises and lowers `go`.

6. The cracker generates, hashes and attempts to match each password in turn.

7. When a password is found, the cracker raises `your_turn` and `match_found`.

8. The driver reads a byte of the password from `password_byte[0..7]`. Passwords are transmitted byte-for-byte from left to right. Each password has exactly 20 bytes of data and is followed by 1 byte of length. For each password, only first _n_ bytes (where _n_ is the length transmitted as the 21st byte) are valid and part of the password.

9. The driver raises and lowers `go`.

10. After 21 repetitions of steps 8 and 9, the cracker continues cracking.

11. When the cracker is finished, it raises `your_turn` without raising `match_found`.
