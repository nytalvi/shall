<p align="center">
  <img src="logo.png" alt="Shall Logo" width="200">
</p>


**Shall** is a command-line tool I made which gives you different hashes for a string or file. Most commands like `sha1sum` just let you calculate one hash at a time, so I made this to make hash calculation a little more efficient. I hope you can find it useful!

## Usage

To generate a hash from a string, run `shall "basil"` (replace "basil" with your string). The output will look something like this:

```
SHA1    : 7377ba3671fcd8c2e6decf7bf5becdaa92ba73dc
SHA256  : 579651b0d574971040b531b66efbc5e607b677aa0d2633ec9ec ...
SHA512  : a80e4e2e0af2d0dbcd448ddccd0787dbfa285fb14ac1f6b393f ...
MD5     : 6862efb4028e93ac23a6f90a9055bae8
# I truncated some of the output for brevity :3
```

You can also use files as inputs, for instance: `shall --file "basil.jpg"`. This will also give a result formatted like the one above.

Note that the outputs are in color, and some terminals don't support that. I plan on implementing `--no-color` at some point in the future hopefully.