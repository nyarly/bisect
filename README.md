# Bisect

`bisect <bitstring>`

Print a subset of the lines on the stardard input
based on the bitstring argument.
The bitstring should be any number of '0' or '1' characters.
For each 0 or 1, bisect will print one half of the lines
(the first half for 0, or the second half for 1)
and then consider the next 0 or 1 and the other half of the file.
Lines are printed in order.

## Example

Consider the file:
```
abrupt
clam
cloth
eggs
elfin
fanatical
future
giants
glow
group
guess
messy
object
puncture
replace
silk
special
sticks
store
thoughtless
troubled
unable
zippy
zoom
```

`bisect 0 < testfile`

will print
```
abrupt
clam
cloth
eggs
elfin
fanatical
future
giants
glow
group
guess
messy
```
(the first half.)

`bisect 101 < testfile`
prints
```
abrupt
clam
cloth
eggs
elfin
fanatical
group
guess
messy
object
puncture
replace
silk
special
sticks
store
thoughtless
troubled
unable
zippy
zoom
```

which omits `future`, `giants` and `glow`.

When experimenting with `bisect`, try

`comm testfile <(bisect BITS < testfile)`

which will product output like

```
		abrupt
		clam
		cloth
		eggs
		elfin
		fanatical
future
giants
glow
		group
		guess
		messy
		object
		puncture
		replace
		silk
		special
		sticks
		store
		thoughtless
		troubled
		unable
		zippy
		zoom
```

## Motivation

It can sometimes be useful to isolate bugs by including
program input until the bug is exhibited.
`bisect` can be used to take a list of inputs,
and quickly search through them until the problem input is located.

In extreme cases, bugs might only be exhibited
when 2 particular inputs appear together.
In this case, `bisect` can be used to isolate one of the inputs,
and then used in concert with `cat` to find its troubled companion.

Specifically, the author has had trouble with test cases in Ruby,
where a single test fails when run with the whole suite but not in isolation,
because another test alters some global state.
`bisect` can be used to identify another test that manipulates the state
which can be instructive as to what's being changed and needs to be properly isolated.

The motivating case for `bisect` was a game that shipped with pre-built libraries,
without which, it crashed, but with all included also crashed. `bisect` helped
identify the subset of shipped libraries
that had to be in `LD_LIBRARY_PATH` to run the game.
