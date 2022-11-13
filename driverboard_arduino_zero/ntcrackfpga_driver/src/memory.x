/* Arduino Zero memory layout (SAMD21x18) */

MEMORY {
    /* K = 1024 bytes */

    /* first 0x2000 of flash used by bootloader -- do not touch! */
    FLASH : ORIGIN = 0x00002000, LENGTH = 256K-0x2000

    RAM : ORIGIN = 0x20000000, LENGTH = 32K
}
