# Declipt

## Special Thanks
…to [WitherOrNot](https://github.com/WitherOrNot) for researching and cracking [Warbird](https://github.com/WitherOrNot/warbird-docs/tree/main).

## Usage
To use Declipt, clone the Git repository, create a folder called `emu64` in the project root, and put `ClipSp.sys` into `emu64`. Make sure you adjust the addresses in `declipt::constants` to match your version of `ClipSp.sys`.

> [!IMPORTANT]
##### For `ClipSp.sys`
You must patch `ClipSp.sys`'s true main entrypoint (you can find this in IDA Pro using CTRL+E) to return `1`. The patched bytes are available in `declipt::hook::CANCEL_DRIVER_ENTRY`. Then, you need to [create fake kernel imports](https://x64dbg.com/blog/2017/06/08/kernel-driver-unpacking.html#faking-the-kernel-imports) for `NTOSKRNL.EXE`, `FLTMGR.SYS`, `HAL.DLL`, and `KSECDD.SYS`. Next, put the fake kernel imports in `emu64`. Finally, set the `0x2000` (File is a DLL) flag in `ClipSp.sys`. You can use [PE Bear](https://github.com/hasherezade/pe-bear) for this.
