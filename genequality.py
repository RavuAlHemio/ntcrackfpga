#!/usr/bin/env python3
import math
import sys
import jinja2

MAX_BYTES = 20
FUNC_TEMPLATE = """
task check_test_result_{{ bytes }}(
    input [{{ 8*bytes - 1 }}:0] expected_out_password);
        if (out_length !== {{ bytes }}) begin
            $display("test %1d failed on length! expected %1d, obtained %1d", current_test, {{ bytes }}, out_length);
            $finish;
        end else if (out_password[{{ 8*bytes - 1 }}:0] !== expected_out_password) begin
            $display("test %1d failed on password! expected %0{{ 2*bytes }}x, obtained %0{{ 2*bytes }}x", current_test, expected_out_password, out_password[{{ 8*bytes - 1 }}:0]);
            $finish;
        end else begin
            $display("test %1d OK", current_test);
        end
endtask
"""

def main():
    if len(sys.argv) != 2:
        print("Usage: genequality.py OUTPUT", file=sys.stderr)
        sys.exit(1)

    template = jinja2.Template(
        FUNC_TEMPLATE,
        undefined=jinja2.StrictUndefined,
        autoescape=None,
    )

    funcs = []
    for byte_count in range(1, MAX_BYTES+1):

        rendered = template.render(
            bytes=byte_count,
        )
        funcs.append(rendered)

    func_str = "".join(funcs)

    with open(sys.argv[1], "w", encoding="us-ascii") as f:
        f.write("// This file has been autogenerated. Manual changes will be lost.\n")
        f.write(func_str)


if __name__ == "__main__":
    main()
