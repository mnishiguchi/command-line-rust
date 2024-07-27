#!/usr/bin/env bash

# Capture the output from the original echo for various inputs so that we can
# compare these to the output from our program. Adopted from:
# https://github.com/kyclark/command-line-rust/blob/10d983f68e84b9c94057da6ecf555bc419e11999/02_echor/mk-outs.sh

OUTDIR="tests/expected"
[[ ! -d "$OUTDIR" ]] && mkdir -p "$OUTDIR"

# one argument with two words
echo "Hello there" >$OUTDIR/hello1.txt

# two arguments separated by more than one space
echo "Hello" "there" >$OUTDIR/hello2.txt

# one argument with two spaces and no newline
echo -n "Hello  there" >$OUTDIR/hello1--no-newline.txt

# two arguments with no newline
echo -n "Hello" "there" >$OUTDIR/hello2--no-newline.txt
