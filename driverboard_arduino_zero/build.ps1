[CmdletBinding()]
Param (
    [switch]
    $NoBuild,

    [switch]
    $NoProgram,

    [switch]
    $DebugBuild,

    [switch]
    $Debugger,

    [string]
    $BinaryName="ntcrackfpga_driver"
)


If (-not $NoBuild)
{
    If ($DebugBuild)
    {
        & cargo build
    }
    Else
    {
        & cargo build --release
    }

    If ($LASTEXITCODE -ne 0)
    {
        Return 1
    }

    $mode = If ($DebugBuild) { "debug" } Else { "release" }
    & rust-objcopy --output-target=binary ".\target\thumbv6m-none-eabi\$mode\$BinaryName" ".\$BinaryName.bin"
    If ($LASTEXITCODE -ne 0)
    {
        Return 1
    }
}

$kilobytes = (Get-Item -LiteralPath ".\$BinaryName.bin").Length / 1024
Write-Output ("{0:#,##0.#} KiB" -f $kilobytes)

If (-not $NoProgram)
{
    & 'C:\Program Files\OpenOCD\bin\openocd.exe' `
        --command "source oocd-program.cfg"
}

If ($Debugger)
{
    & 'C:\Program Files\arm-gcc\bin\arm-none-eabi-gdb.exe' `
        -ex "target extended-remote \\.\$DebugPort" `
        -ex "monitor jtag_scan" `
        -ex "attach 1" `
        ".\target\thumbv7m-none-eabi\release\$BinaryName"
}
