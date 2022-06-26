# Overview

I've often written little scripts to recursively do a bunch of stuff
to a directory of files.

This is my attempt at a little more general purpose solution, and
hopefully easier to use than shell/find/xargs.

# Example

This is how to convert a bunch of files from .tiff to .png to a
different output directory:

    batch-command-runner ".*\.tif" "mkdir -p '~/tmp/photos/@dirname@'; convert '{}' '~/tmp/photos/@dirname@/@filenamebase@.png'"

# TODO

* implement "multi" job runner
* add more special keyword substitutions

