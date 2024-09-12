# Declipt

## Special Thanks
…to [WitherOrNot](https://github.com/WitherOrNot) for researching and cracking [Warbird](https://github.com/WitherOrNot/warbird-docs/tree/main).

## Usage
To use Declipt, clone the Git repository and `ClipSp.sys` into a folder called `maps64` in the project root.

You will then need to add the following files to `maps64` (find them yourself):
- `BOOTVID.DLL`
- `CI.DLL`
- `CNG.SYS`
- `EXT-MS-WIN-CI-XBOX-L1-1-0.DLL`
- `EXT-MS-WIN-FS-CLFS-L1-1-0.DLL`
- `EXT-MS-WIN-NTOS-CLIPSP-L1-1-0.DLL`
- `EXT-MS-WIN-NTOS-KCMINITCFG-L1-1-0.DLL`
- `EXT-MS-WIN-NTOS-KSECURITY-L1-1-1.DLL`
- `EXT-MS-WIN-NTOS-KSR-L1-1-3.DLL`
- `EXT-MS-WIN-NTOS-PROCESSPARAMETERS-L1-1-0.DLL`
- `EXT-MS-WIN-NTOS-STATESEPARATION-L1-1-0.DLL`
- `EXT-MS-WIN-NTOS-TM-L1-1-0.DLL`
- `EXT-MS-WIN-NTOS-TRACE-L1-1-0.DLL`
- `EXT-MS-WIN-NTOS-UCODE-L1-1-0.DLL`
- `EXT-MS-WIN-NTOS-WERKERNEL-L1-1-1.DLL`
- `FLTMGR.SYS`
- `HAL.DLL`
- `KDCOM.DLL`
- `KSECDD.SYS`
- `MSRPC.SYS`
- `NTOSKRNL.EXE`
- `PSHED.DLL`

> [!IMPORTANT]
##### For imported binaries
Follow [the instructions here](https://x64dbg.com/blog/2017/06/08/kernel-driver-unpacking.html#faking-the-kernel-imports) for all binaries (excluding `ClipSp.sys`) in `maps64`.
##### For `ClipSp.sys`
You must patch `ClipSp.sys`'s `DriverEntry` (also known as `DllMain`) to return `1`. The patched bytes are available in `declipt::hook::CANCEL_DRIVER_ENTRY`. You also need to set the `0x2000` (File is a DLL) flag in `ClipSp.sys`. You can use [PE Bear](https://github.com/hasherezade/pe-bear) for this.
