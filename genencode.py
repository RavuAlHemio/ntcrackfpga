#!/usr/bin/env python3
import math
import sys
from typing import NamedTuple
import jinja2


PASSWORD_CHARS = 20
TEMPLATE = """
function [511:0] password_to_md4_data(
    input [{{ password_chars * 8 - 1 }}:0] in,
    input [{{ len_topbit }}:0] len);
    begin
        case (len)
            {%- for n in range(password_chars+1) %}
                {{ n }}: password_to_md4_data = {
                    {#- conversion to UTF-16 #}
                    {%- for i in range(n)|reverse %}
                        {%- if not loop.first %}, {% endif -%}
                        in[{{ 8*i + 7 }}:{{ 8 * i }}], 8'h0
                    {%- endfor %}

                    {#- padding #}
                    {%- if n > 0 %}, {% endif -%}
                    8'h80, {{ n|zero_padding_bit_count }}'h0
                    {%- for b in n|utf16_bit_count_le_bytes -%}
                    , 8'h{{ b|hex }}
                    {%- endfor -%}
                };
            {%- endfor %}
            default: password_to_md4_data = 512'h00;
        endcase
    end
endfunction
"""


def zero_padding_bit_count(ascii_byte_count: int) -> int:
    utf16_byte_count = ascii_byte_count * 2
    # 64 bytes per chunk minus 8 bytes length minus 1 byte 0x80 minus data bytes
    zero_padding_byte_count = 64 - (8 + 1 + utf16_byte_count)
    return zero_padding_byte_count * 8


def utf16_bit_count_le_bytes(ascii_byte_count: int) -> bytes:
    utf16_byte_count = ascii_byte_count * 2
    utf16_bit_count = utf16_byte_count * 8
    return utf16_bit_count.to_bytes(8, "little")


def hex_filter(i: int) -> str:
    return f"{i:x}"


def main():
    if len(sys.argv) != 2:
        print("Usage: genencode.py OUTPUT", file=sys.stderr)
        sys.exit(1)

    input_topbit = PASSWORD_CHARS * 8 - 1
    len_topbit = math.ceil(math.log2(input_topbit + 1))

    env = jinja2.Environment(undefined=jinja2.StrictUndefined)
    env.filters["zero_padding_bit_count"] = zero_padding_bit_count
    env.filters["utf16_bit_count_le_bytes"] = utf16_bit_count_le_bytes
    env.filters["hex"] = hex_filter
    tpl = env.from_string(TEMPLATE)
    output = tpl.render(
        password_chars=PASSWORD_CHARS,
        len_topbit=len_topbit,
    )

    with open(sys.argv[1], "w", encoding="us-ascii") as f:
        f.write("// This file has been autogenerated. Manual changes will be lost.\n")
        f.write(output)


if __name__ == "__main__":
    main()
