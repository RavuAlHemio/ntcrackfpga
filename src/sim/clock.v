module clock(output clock_signal);

reg internal_clock;

assign clock_signal = internal_clock;

initial begin
    forever begin
        #1
        internal_clock = ~internal_clock;
    end
end

endmodule
