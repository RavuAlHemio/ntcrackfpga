#!/usr/bin/env python3
import hashlib

TEST = b"f\0a\0r\0t\0"


def md4pad(what: bytes) -> bytes:
    # get length of bytes
    len_bytes = len(what)
    len_bits = len_bytes * 8

    # pad to multiple of 56 mod 64 bytes
    padded = bytearray(what)
    padded.extend(b"\x80")
    while len(padded) % 64 != 56:
        padded.extend(b"\x00")

    # append length as 64-bit little-endian number of bits
    len_bits_as_bytes = len_bits.to_bytes(8, "little")
    padded.extend(len_bits_as_bytes)

    assert len(padded) % 64 == 0

    return bytes(padded)


def md4(what: bytes) -> str:
    padded = md4pad(what)

    assert len(padded) % 64 == 0

    # our operations
    f = lambda x, y, z: (x & y) | ((x ^ 0xFFFFFFFF) & z)
    g = lambda x, y, z: (x & y) | (x & z) | (y & z)
    h = lambda x, y, z: x ^ y ^ z
    def rol(val, count):
        print(f"rolval={(val & 0xFFFFFFFF):08x}")
        ret = (((val & 0xFFFFFFFF) << count)| ((val & 0xFFFFFFFF) >> (32 - count))) & 0xFFFFFFFF
        print(f"rolret={ret:08x}")
        return ret
    be2le = lambda val: ((val & 0x000000FF) << 24) | ((val & 0x0000FF00) << 8) | ((val & 0x00FF0000) >> 8) | ((val & 0xFF000000) >> 24)

    i = 0
    a = 0x67452301
    b = 0xefcdab89
    c = 0x98badcfe
    d = 0x10325476

    while i < len(padded):
        what_hex = "".join(f"{b:02x}" for b in padded)
        print(f"DATA: {what_hex}")
        print(f"START: {a:08x} {b:08x} {c:08x} {d:08x}")
        old_a = a
        old_b = b
        old_c = c
        old_d = d

        chunk: list[int] = []
        for j in range(0, 64, 4):
            chunk.append(int.from_bytes(padded[i+j:i+j+4], "little", signed=False))

        # F
        a = rol(a + f(b, c, d) + chunk[ 0],  3)
        d = rol(d + f(a, b, c) + chunk[ 1],  7)
        c = rol(c + f(d, a, b) + chunk[ 2], 11)
        b = rol(b + f(c, d, a) + chunk[ 3], 19)
        a = rol(a + f(b, c, d) + chunk[ 4],  3)
        d = rol(d + f(a, b, c) + chunk[ 5],  7)
        c = rol(c + f(d, a, b) + chunk[ 6], 11)
        b = rol(b + f(c, d, a) + chunk[ 7], 19)
        a = rol(a + f(b, c, d) + chunk[ 8],  3)
        d = rol(d + f(a, b, c) + chunk[ 9],  7)
        c = rol(c + f(d, a, b) + chunk[10], 11)
        b = rol(b + f(c, d, a) + chunk[11], 19)
        a = rol(a + f(b, c, d) + chunk[12],  3)
        d = rol(d + f(a, b, c) + chunk[13],  7)
        c = rol(c + f(d, a, b) + chunk[14], 11)
        b = rol(b + f(c, d, a) + chunk[15], 19)

        print(f"POST-F: {a:08x} {b:08x} {c:08x} {d:08x}")

        # G
        a = rol(a + g(b, c, d) + chunk[ 0] + 0x5A827999,  3)
        d = rol(d + g(a, b, c) + chunk[ 4] + 0x5A827999,  5)
        c = rol(c + g(d, a, b) + chunk[ 8] + 0x5A827999,  9)
        b = rol(b + g(c, d, a) + chunk[12] + 0x5A827999, 13)
        a = rol(a + g(b, c, d) + chunk[ 1] + 0x5A827999,  3)
        d = rol(d + g(a, b, c) + chunk[ 5] + 0x5A827999,  5)
        c = rol(c + g(d, a, b) + chunk[ 9] + 0x5A827999,  9)
        b = rol(b + g(c, d, a) + chunk[13] + 0x5A827999, 13)
        a = rol(a + g(b, c, d) + chunk[ 2] + 0x5A827999,  3)
        d = rol(d + g(a, b, c) + chunk[ 6] + 0x5A827999,  5)
        c = rol(c + g(d, a, b) + chunk[10] + 0x5A827999,  9)
        b = rol(b + g(c, d, a) + chunk[14] + 0x5A827999, 13)
        a = rol(a + g(b, c, d) + chunk[ 3] + 0x5A827999,  3)
        d = rol(d + g(a, b, c) + chunk[ 7] + 0x5A827999,  5)
        c = rol(c + g(d, a, b) + chunk[11] + 0x5A827999,  9)
        b = rol(b + g(c, d, a) + chunk[15] + 0x5A827999, 13)

        # H
        a = rol(a + h(b, c, d) + chunk[ 0] + 0x6ED9EBA1,  3)
        d = rol(d + h(a, b, c) + chunk[ 8] + 0x6ED9EBA1,  9)
        c = rol(c + h(d, a, b) + chunk[ 4] + 0x6ED9EBA1, 11)
        b = rol(b + h(c, d, a) + chunk[12] + 0x6ED9EBA1, 15)
        a = rol(a + h(b, c, d) + chunk[ 2] + 0x6ED9EBA1,  3)
        d = rol(d + h(a, b, c) + chunk[10] + 0x6ED9EBA1,  9)
        c = rol(c + h(d, a, b) + chunk[ 6] + 0x6ED9EBA1, 11)
        b = rol(b + h(c, d, a) + chunk[14] + 0x6ED9EBA1, 15)
        a = rol(a + h(b, c, d) + chunk[ 1] + 0x6ED9EBA1,  3)
        d = rol(d + h(a, b, c) + chunk[ 9] + 0x6ED9EBA1,  9)
        c = rol(c + h(d, a, b) + chunk[ 5] + 0x6ED9EBA1, 11)
        b = rol(b + h(c, d, a) + chunk[13] + 0x6ED9EBA1, 15)
        a = rol(a + h(b, c, d) + chunk[ 3] + 0x6ED9EBA1,  3)
        d = rol(d + h(a, b, c) + chunk[11] + 0x6ED9EBA1,  9)
        c = rol(c + h(d, a, b) + chunk[ 7] + 0x6ED9EBA1, 11)
        b = rol(b + h(c, d, a) + chunk[15] + 0x6ED9EBA1, 15)

        # increment!
        a = (a + old_a) & 0xFFFFFFFF
        b = (b + old_b) & 0xFFFFFFFF
        c = (c + old_c) & 0xFFFFFFFF
        d = (d + old_d) & 0xFFFFFFFF

        print(f"newstate: {a:08x} {b:08x} {c:08x} {d:08x}")

        i += 64

    # swap endianness
    a_le, b_le, c_le, d_le = be2le(a), be2le(b), be2le(c), be2le(d)
    return f"{a_le:08x}{b_le:08x}{c_le:08x}{d_le:08x}"


def main():
    import binascii, sys
    if len(sys.argv) > 1:
        if sys.argv[1] == "pad":
            pad_what = sys.argv[2] if len(sys.argv) > 2 else ""
            print(binascii.hexlify(md4pad(pad_what.encode("us-ascii"))).decode("us-ascii"))
            return
        if sys.argv[1] == "ref":
            ref_what = sys.argv[2] if len(sys.argv) > 2 else ""
            reference = hashlib.new("md4")
            reference.update(ref_what.encode("us-ascii"))
            print(reference.hexdigest().upper())
            return
        if sys.argv[1] == "run":
            run_what = sys.argv[2].encode("us-ascii") if len(sys.argv) > 2 else TEST
            # keep going

    reference = hashlib.new("md4")
    reference.update(run_what)
    ref_digest = reference.hexdigest()

    testy = md4(run_what)

    print(f"ref: {ref_digest}")
    print(f"tst: {testy}")
    if ref_digest == testy:
        print("SAME")
    else:
        print("DIFFERENT")


if __name__ == "__main__":
    main()
