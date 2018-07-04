# BK168xB power supply Rust bindings

The BK Precision BK168xB family of supplies consists of the 1685B, the 1687B,
and the 1688B.  These supplies feature USB control; however,
manufacturer-provided control software is only available under Windows.

This project will provide two main elements:

-   A library with control bindings for these supplies
-   A basic curses-based application for supply control (inspired by
    [`psucontrol`](https://github.com/TheUbuntuGuy/psucontrol))
