# Experiments with nRF52840-dongle

## Debug

A J-Link EDU is úsed to develop this software. The SWD interface is used.

[JLinkGDBServer] from Segger is used to debug, see the `jlinkgdb` shell script
on how JLinkGDBServer is invoked.

Start the GDB server with `jlinkgdb`.

```
$ ./jlinkgdb
```

Then run the program

```
$ cargo run
```

cargo will use the run definition found in `.cargo/config` to launch `gdb` with
the `jlink.gdb` script file.

## License

Licensed under the MIT license. See LICENSE.
