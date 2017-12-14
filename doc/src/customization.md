## Makefile

The main point of customization is your `Makefile`.  The following Make variables are supported:

  - `tool_dir` (required): The directory that contains the tools.
  - `metadata`: This can contain either:
      - `-M title="<title>"`
      - `-M author="<author>"` (you can repeat this flag multiple times for multiple authors)
  - `pandoc_args`: Arguments given to all Pandoc invocations.
  - `latex_documentclass`: LaTeX document class (defaults to `book`).
  - `latex_pandoc_args`: Arguments given to the Pandoc invocation that generates LaTeX.
  - `html_pandoc_args`: Arguments given to all Pandoc invocations that generate HTML.

Pandoc offers [a lot of ways](https://pandoc.org/MANUAL.html) to customize your output, many of which can have dramatic effects on the end result.  If for some reason one of the flags interacts poorly with the `book-mk` system, please report a bug so we can either document it as a limitation or (better yet) find a workaround!

For more advanced customizations, you'd want to read [@Sec:internals].

## Environment variables

When building, the following variables are used to determine where the respective executables are found.  This can be useful if you have, e.g. `pandoc` installed at an unconventional location that is not on `PATH`.

  - `CARGO`
  - `INKSCAPE`
  - `LATEXMK`
  - `PANDOC`
  - `PANDOC_CITEPROC`
  - `PANDOC_CROSSREF`

By default, they simply default to their unqualified executable names (i.e. `PANDOC` defaults to just `pandoc`, etc).
