#!/usr/bin/env python3
import math
import os
import sys
from typing import NamedTuple
import jinja2


MUX_READ_TEMPLATE = """
// input [{{ mux.full_size - 1 }}:0] full
// input [{{ mux.index_size - 1 }}:0] index
// output [{{ mux.slice_size - 1}}:0] slice
`define MUX_READ_{{ mux.slice_size }}_OF_{{ mux.full_size }}(full, index, slice) \\
    case (index) \\
        {%- for i in range(mux.max_index + 1) %}
        {{ i }}: slice <= full[{{ mux.slice_size*(i+1) - 1 }}:{{ mux.slice_size*i }}]; \\
        {%- endfor %}
        default: slice <= 'hX; \\
    endcase
"""


MUX_WRITE_TEMPLATE = """
// output [{{ mux.full_size - 1 }}:0] full
// input [{{ mux.index_size - 1 }}:0] index
// input [{{ mux.slice_size - 1}}:0] slice
`define MUX_WRITE_{{ mux.slice_size }}_OF_{{ mux.full_size }}(full, index, slice) \\
    case (index) \\
        {%- for i in range(mux.max_index + 1) %}
        {{ i }}: full[{{ mux.slice_size*(i+1) - 1 }}:{{ mux.slice_size*i }}] <= slice; \\
        {%- endfor %}
        default: ; \\
    endcase
"""


class Multiplexer(NamedTuple):
    slice_size: int
    full_size: int

    @property
    def max_index(self):
        return self.full_size // self.slice_size - 1

    @property
    def index_size(self):
        return math.ceil(math.log2(self.max_index + 1))


def main():
    if len(sys.argv) != 2:
        print("Usage: genmux.py OUTPUT", file=sys.stderr)
        sys.exit(1)

    multiplexers = [
        Multiplexer(
            slice_size=128,
            full_size=128*128,
        ),
        Multiplexer(
            slice_size=8,
            full_size=160,
        ),
    ]

    read_tpl = jinja2.Template(
        MUX_READ_TEMPLATE,
        undefined=jinja2.StrictUndefined,
    )
    write_tpl = jinja2.Template(
        MUX_WRITE_TEMPLATE,
        undefined=jinja2.StrictUndefined,
    )

    all_muxes = []
    for mux in multiplexers:
        all_muxes.append(read_tpl.render(mux=mux))
        all_muxes.append(write_tpl.render(mux=mux))

    with open(sys.argv[1], "w", encoding="us-ascii") as f:
        f.write("// This file has been autogenerated. Manual changes will be lost.\n")
        f.write("".join(all_muxes))

if __name__ == "__main__":
    main()
