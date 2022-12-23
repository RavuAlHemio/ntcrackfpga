# VersaBoard and pin banks

Since we will be doing +3.3V I/O, we must ensure that the ECP5 evaluation board is ready to join in.

According to Table 2.1 in the _ECP5 Evaluation Board User's Guide_ (Lattice FPGA-EB-02017-1.2), the following I/O voltages are preconfigured for each pin bank:

| bank | default V | change?   |
| ---- | --------- | --------- |
| 0    | +3.3V     | JP10      |
| 1    | **+2.5V** | resistors |
| 2    | +3.3V     | resistors |
| 3    | +3.3V     | resistors |
| 6    | +3.3V     | resistors |
| 7    | +3.3V     | JP11      |
| 8    | +3.3V     | resistors |

This means that we should avoid the pins in bank 1.

The VersaBoard pin bank assignment can be collected from:

* tables 5.1 and 5.2 from the _ECP5 Evaluation Board User's Guide_

* the _ECP5UM5G-85 Pinout_ CSV file provided by Lattice (consult columns _CABGA381_ and _Bank_)

| Versa connector | Versa pin | FPGA ball   | pin bank | function |
| --------------- | --------- | ----------- | -------- | -------- |
| J39             | 1         | GND         | n/a      | n/a      |
| J39             | 2         | NC          | n/a      | n/a      |
| J39             | 3         | EXPCON_2V5  | n/a      | n/a      |
| J39             | 4         | D15         | 1        | PT107A   |
| J39             | 5         | B15         | 1        | PT105A   |
| J39             | 6         | C15         | 1        | PT105B   |
| J39             | 7         | B13         | 1        | PT78A    |
| J39             | 8         | B20         | 1        | PT121B   |
| J39             | 9         | D11         | 1        | PT69A    |
| J39             | 10        | E11         | 1        | PT69B    |
| J39             | 11        | B12         | 1        | PT71A    |
| J39             | 12        | C12         | 1        | PT71B    |
| J39             | 13        | D12         | 1        | PT74A    |
| J39             | 14        | E12         | 1        | PT74B    |
| J39             | 15        | C13         | 1        | PT78B    |
| J39             | 16        | D13         | 1        | PT80A    |
| J39             | 17        | E13         | 1        | PT80B    |
| J39             | 18        | A14         | 1        | PT83A    |
| J39             | 19        | A9          | 0        | PT60A    |
| J39             | 20        | B10         | 0        | PT60B    |
| J39             | 21        | 5VIN        | n/a      | n/a      |
| J39             | 22        | GND         | n/a      | n/a      |
| J39             | 23        | EXPCON_2V5  | n/a      | n/a      |
| J39             | 24        | GND         | n/a      | n/a      |
| J39             | 25        | +3.3V       | n/a      | n/a      |
| J39             | 26        | GND         | n/a      | n/a      |
| J39             | 27        | +3.3V       | n/a      | n/a      |
| J39             | 28        | GND         | n/a      | n/a      |
| J39             | 29        | E7          | 0        | PT9A     |
| J39             | 30        | GND         | n/a      | n/a      |
| J39             | 31        | A11         | 0        | PT63B    |
| J39             | 32        | GND         | n/a      | n/a      |
| J39             | 33        | A19         | 1        | PT121A   |
| J39             | 34        | GND         | n/a      | n/a      |
| J39             | 35        | EXPCON_3V3  | n/a      | n/a      |
| J39             | 36        | GND         | n/a      | n/a      |
| J39             | 37        | EXPCON_3V3  | n/a      | n/a      |
| J39             | 38        | GND         | n/a      | n/a      |
| J39             | 39        | EXPCON_3V3  | n/a      | n/a      |
| J39             | 40        | GND         | n/a      | n/a      |
| J40             | 1         | K2          | 6        | PL53A    |
| J40             | 2         | GND         | n/a      | n/a      |
| J40             | 3         | A15         | 1        | PT103A   |
| J40             | 4         | F1          | 6        | PL47B    |
| J40             | 5         | H2          | 6        | PL47C    |
| J40             | 6         | G1          | 6        | PL47D    |
| J40             | 7         | J4          | 6        | PL50A    |
| J40             | 8         | J5          | 6        | PL50B    |
| J40             | 9         | J3          | 6        | PL50C    |
| J40             | 10        | K3          | 6        | PL50D    |
| J40             | 11        | L4          | 6        | PL56C    |
| J40             | 12        | L5          | 6        | PL56D    |
| J40             | 13        | M4          | 6        | PL83A    |
| J40             | 14        | N5          | 6        | PL83B    |
| J40             | 15        | N4          | 6        | PL83C    |
| J40             | 16        | P5          | 6        | PL83D    |
| J40             | 17        | N3          | 6        | PL86A    |
| J40             | 18        | M3          | 6        | PL86B    |
| J40             | 19        | GND         | n/a      | n/a      |
| J40             | 20        | EXPCON_3V3  | n/a      | n/a      |
| J40             | 21        | K5          | 6        | PL56B    |
| J40             | 22        | GND         | n/a      | n/a      |
| J40             | 23        | M5          | 6        | PL77A    |
| J40             | 24        | GND         | n/a      | n/a      |
| J40             | 25        | L3          | 6        | PL86C    |
| J40             | 26        | GND         | n/a      | n/a      |
| J40             | 27        | N2          | 6        | PL89A    |
| J40             | 28        | M1          | 6        | PL89B    |
| J40             | 29        | L2          | 6        | PL86D    |
| J40             | 30        | GND         | n/a      | n/a      |
| J40             | 31        | L1          | 6        | PL89C    |
| J40             | 32        | N1          | 6        | PL89D    |
| J40             | 33        | C14         | 1        | PT83B    |
| J40             | 34        | GND         | n/a      | n/a      |
| J40             | 35        | P1          | 6        | PL92A    |
| J40             | 36        | E14         | 1        | PT85B    |
| J40             | 37        | D14         | 1        | PT85A    |
| J40             | 38        | CARDSEL     | n/a      | n/a      |
| J40             | 39        | K4          | 6        | PL56A    |
| J40             | 40        | GND         | n/a      | n/a      |
