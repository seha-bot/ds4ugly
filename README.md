# ds4ugly
DualShock wireless emulator for windows

## Why?
Many new games support the DualShock 4 controller as an input method.
However, in order for it to work correctly it needs to be plugged in with a cable.
It will not be recognized when connected with Bluetooth.

This program allows you to connect a DualShock 4 with Bluetooth and it will emulate a USB device
with the same vendor id as a legit controller. This will allow you to play without being plugged in!

There are other tools that do similar things (like DS4Windows), but I think it's bloated and it
maps your controller to an Xbox control scheme. Because of that, games show you UI with Xbox controls
and I think that's pretty annoying.

## How to run?
1. Download the vigem driver from https://github.com/ViGEm/ViGEmBus/releases
2. Setup rust on your machine and simply run this:

```bash
cargo build --release
```

## Future
Currently, in order for it to work you need the vigem driver
and I don't like depending on other people's work.
I will make a driver specifically for this project, or find a way to emulate it in a different way.
Maybe even make it cross-platform :)

Also, the project depends on gilrs-core for the base bluetooth controller interaction.
Gilrs doesn't recognise events with multiple d-pad buttons pressed, so you can't press UP and LEFT
in the same time. Gilrs also doesn't register events for pressing the touchbar and ps button, so I
will look into an alternative or make my own :/
