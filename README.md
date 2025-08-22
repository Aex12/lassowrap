# lassowrap
```
Usage: lassowrap.exe [OPTIONS] -- <EXECUTABLE> [EXECUTABLE_PARAMETERS]...

Arguments:
  <EXECUTABLE>                The program to run
  [EXECUTABLE_PARAMETERS]...  Arguments to pass to the program (captures everything that follows)

Options:
  -p, --priority <PRIORITY>  Process priority [possible values: low, belownormal, normal, abovenormal, high, realtime]
  -a, --affinity <AFFINITY>  CPU affinity mask (e.g. 0xFFFF000)
  -v, --verbose              Verbose output
  -h, --help                 Print help
```