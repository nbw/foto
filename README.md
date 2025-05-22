# Foto

CLI tool for various image processing functions (contrast, saturdation, etc.)

This tool is very WIP for the time being is purely being used for educational purposes.


<!-- start: CLI USAGE -->

# Command-Line Help for `foto`

This document contains the help content for the `foto` command-line program.

**Command Overview:**

* [`foto`↴](#foto)
* [`foto contrast`↴](#foto-contrast)
* [`foto saturation`↴](#foto-saturation)

## `foto`

A CLI tool for image processing

**Usage:** `foto <COMMAND>`

###### **Subcommands:**

* `contrast` — Adjust the contrast of an image
* `saturation` — 



## `foto contrast`

Adjust the contrast of an image

**Usage:** `foto contrast [OPTIONS] --input <INPUT> --output <OUTPUT> --ratio <RATIO>`

###### **Options:**

* `-i`, `--input <INPUT>` — Path to the input image
* `-o`, `--output <OUTPUT>` — Path to the output image
* `-r`, `--ratio <RATIO>` — Contrast ratio (must be greater than 0)
* `-t`, `--threshold <THRESHOLD>` — Threshold value (between 0 and 256)

  Default value: `128.0`



## `foto saturation`

**Usage:** `foto saturation [OPTIONS] --input <INPUT> --output <OUTPUT> --amount <AMOUNT>`

###### **Options:**

* `-i`, `--input <INPUT>` — Path to the input image
* `-o`, `--output <OUTPUT>` — Path to the output image
* `-a`, `--amount <AMOUNT>` — Saturation amount (must be greater than 0)
* `-t`, `--type <SAT_TYPE>` — Saturation type (hsv or luma)

  Default value: `hsv`



<hr/>

<small><i>
    This document was generated automatically by
    <a href="https://crates.io/crates/clap-markdown"><code>clap-markdown</code></a>.
</i></small>

<!-- end: CLI USAGE -->