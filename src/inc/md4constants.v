`define SET_MD4_INITIAL(a, b, c, d) \
    a <= 32'h67452301; \
    b <= 32'hefcdab89; \
    c <= 32'h98badcfe; \
    d <= 32'h10325476;
