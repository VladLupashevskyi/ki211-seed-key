# Seed key algorithm for Mercedes 211/209/203/171 instrument clusters

Implementation of seed key algorithm for KI211/KI209/KI203/KI171 etc. clusters in Rust.

Although seed given by most of these clusters is 8 bytes, only 4 bytes are used for key calculation, therefore it's possible that this algorithm will work for older KI203 cluster that return 4 bytes seed.

The produced key is 4 bytes long, however some clusters require access level (1 byte) and dongle ID (2 bytes) present in response. It looks like dongle ID can be any value and it's not used by cluster at all and just gets stored in EEPROM.

For calculation you would need to have a root key (4 bytes) which is different for each cluster software version.

Some examples are demonstrated in tests.

The root key can be bruteforced if at least one seed key pair is known.

FOR EDUCATIONAL PURPOSES ONLY

## License

MIT

This software does not include or require copyrighted or proprietary files.

_When interacting with this repository (PR, issues, comments), please avoid including copyrighted/proprietary files, as they will be removed without notice._
