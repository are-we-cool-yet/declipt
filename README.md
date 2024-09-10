# Declipt

## Usage
To use Declipt, clone the Git repository, and copy `maps64` (found [here](https://github.com/sha0coder/scemu/tree/main)) and `ClipSp.sys` into the project root.

You will then need to add the following files to `maps64` (find them yourself):
- `fltMgr.sys`
- `hal.sys`
- `kernel32.dll`
- `ksecdd.sys`
- `ntdll.dll`
- `ntoskrnl.exe`

> [!IMPORTANT]
Because of some weird shenanigans that are beyond me, you must rename any `.exe` or `.sys` dependencies in `map64` to `.exe.dll` or `.sys.dll` respectively.
