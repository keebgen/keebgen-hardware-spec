# Keebgen hardware spec proposal

## Required information

These are things the keyboard will not even perform basic functions without. All
of these MUST be provided and can not be assumed. This should be as minimal as
possible while allowing tooling, be it firmware, or conversion tools, to make
the assumptions from provided information if not provided in a further "soft
spec".

### Pinout or footprint

Pins need provided. We can support either bare pins on bare microcontrollers, or
indexed/converted pins such as QMK's `CONVERT_TO` or KMK's `quickpin`
specifications. Footprints allow us to follow a unified process for tooling so
all microcontrollers can be output, and let their tooling take care of
conversions as they see fit. This should reduce the redundant code substantially
for some footprints, but does not describe all keyboards such as integrated
microcontrollers, or where only one controller works on a footprint, so both
need to be supported within the spec.

Things that should be accepted for pin uses
---
All keyboards

* col pins
* row pins

OR

* direct pins


## Nice to have pins

Splits are not *technically* required to be supported. Split boards that do not
require a primary half should be able to boot as a single piece ignorant of the
other half. This will not work for pin extender boards.

* rgb pin
* serial pin (used for split communication)
* i2c (if not defined by footprint or other custom thing needed)

## Assumption override

Keyboards can assume a lot of things, but those things aren't always correct.
Overrides allow us to correct for these things, and are part of all functional
keyboard firmwares where the hardware demands an override.

* coordinate mapping (the keyboard isn't a square)
* diode direction (row2col/col2row)

# Other thoughts

This is the *absolute* minimum it takes to boot a board. This does not define
every single thing that a keyboard can do, but the absolute bare minimum to get
it to have a physical understanding of what keys could be assigned to. Tooling
to/from each firmware can add their own assumptions, have their own overrides,
and will differ in what they care about, but for the bare minimum understanding
of what a keyboard is should be in the core "hard" spec.
