# Bisect

`bisect <bitstring>`

Print a subset of the lines on the stardard input
based on the bitstring argument.  The bitstring should be any
number of '0' or '1' characters.
The first character will print half the lines of STDIN
(the first half for 0, or the second half for 1).  Each additional
digit will "add in" half of the remaining lines, with a 0 or 1 indicating
which half.

By progressively adding digits you can bisectively rebuild the original
input, excluding a smaller subset each time.

This can be useful for debugging: for example including progressively
larger subsets of a test suite, included libraries, or other list
where an unknown element of the list is causing a problem.

## Example

Consider this eight-line text file:
```
$ cat testfile
01-alpha
02-bravo
03-charlie
04-delta
05-echo
06-foxtrot
07-golf
08-hotel
```

Running `bisect 0 < testfile` will print the first half of the input:
```
$ bisect 0 < testfile
01-alfa
02-bravo
03-charlie
04-delta
```

Suppose that worked, and you want to try including two more lines.

Running `bisect 00 < testfile`
will print the first quarter of the file:
```
$ bisect 00 < testfile
01-alfa
02-bravo
```

If that didn't work, you might try:

`bisect 01 < testfile`

which prints the second quarter:

```
$ bisect 01 < testfile
03-charlie
04-delta
```

Proceeding to
`bisect 010 < testfile` would print the 3rd 8th
(just line 3, in this case):
```
$ bisect 010 < testfile
03-charlie
```

If that works, you have identified `03-foxtrot` as the "problematic"
entry in your original list.

The general principle is to use `bisect`
in some kind of shell one liner
or throwaway script
and run it successively,
adding 0s when things work,
and toggling the last 0 to a 1 when they break.

### Experimenting

The behavior of `bisect` can be unintuitive at first.
To get a feel for it,
it can help to run through a sample file,
building up the selection string to see what gets selected.

For a long list, the `comm` (common entries) command combines
well with `bisect` to help identify the excluded line(s).

```
$ comm testfile2 <(bisect 101 < testfile2)`
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

### Invert

Because the case of
needing a _growing_ list of test entries
is a common one,
`bisect` has an `--invert` flag,
which simply suppresses the selected lines,
rather than selectively printing them.

Compare
```
$ bisect 01 < testfile
03-charlie
04-delta
```

with

```
$ bisect -i 01 < testfile
01-alfa
02-bravo
05-echo
06-foxtrot
07-golf
08-hotel
```

## Motivation

Including progressively larger inputs until a bug is reproduced
is often a useful way to isolate bugs.
`bisect` can be used to take a list of inputs
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
