# schnell + snail = schnail [![Build Status](https://travis-ci.org/kmein/schnail.svg?branch=master)](https://travis-ci.org/kmein/schnail)
*Alex Randolph's board game »[Tempo, kleine Schnecke](https://de.wikipedia.org/wiki/Tempo,_kleine_Schnecke)« in Rust*

This program is a simulation of the game's simple version. Its rules are:

* There are six snails with different colours.
* Two six-sided, coloured dice are thrown.
* The snails bearing the colours of the dice go forward one step. If the same
  colour comes up twice, the so-coloured snail goes forward two steps.
* The snail that is first to cross the finish line wins the game.

You can run it with `cargo run`.
