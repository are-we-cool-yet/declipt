# Declipt

## Special Thanks
…to [WitherOrNot](https://github.com/WitherOrNot) for researching and cracking [Warbird](https://github.com/WitherOrNot/warbird-docs/tree/main).

## Usage
To use Declipt, clone the Git repository and put `ClipSp.sys` into the project root. Make sure you adjust the addresses in `declipt::constants` to match your version of `ClipSp.sys`.

> [!IMPORTANT]
##### For `ClipSp.sys`
You must patch `ClipSp.sys`'s true main entrypoint (you can find this in IDA Pro using CTRL+E) to return `1`. The patched bytes are available in `declipt::hook::CANCEL_DRIVER_ENTRY`. Then, you need to delete the import table. I used [CFF Explorer](https://ntcore.com/explorer-suite/) to do this. You also need to set the `0x2000` (File is a DLL) flag in `ClipSp.sys`. You can use [PE Bear](https://github.com/hasherezade/pe-bear) for this.
