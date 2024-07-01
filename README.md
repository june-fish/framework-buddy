# Framework Buddy

A Linux GUI app for managing EC settings on the Framework laptop. **Currently alpha quality at best, you have been warned.**

Currently requires root and secure boot to be disabled to work on AMD platforms (will run with pkexec by default), until the release of kernel 6.10, which should include a CrosEc driver that supports AMD frameworks. This app will not be considered "finished"/ready to be packaged until it no longer requires Portio. I do not have an Intel Framework to test if CrosEc works on those, but there is no reason why it shouldn't.

## Build/Install

Requires polkit and a rust toolchain

1. Build with `make release`
2. Install with `sudo make install`

To uninstall run `sudo make uninstall`

## Acknowledgements

[Relm4](https://relm4.org) for the GUI framework

[libhelium](https://github.com/tau-OS/libhelium) for the components

[hydrogen](https://github.com/tau-OS/tau-hydrogen) for the icons

[Lea](http://github.com/lleyton) for putting up with my dumb questions
