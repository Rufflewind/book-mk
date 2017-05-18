`book-mk` is a set of scripts for building books from Markdown source files.  Currently, HTML and PDF are supported.

`book-mk` is not meant to be installed as a package.  Instead, it is intended to be embedded inside a subdirectory of your book's source tree.  You can use [Git submodules](https://git-scm.com/book/en/v2/Git-Tools-Submodules) for this but it is not required and won't be explained here.

## Requirements

First of all, you need a Unix-like system, because the build system relies heavily on makefiles.  Additionally, the following tools are needed:

  - [GNU Make 4.2.\*](https://www.gnu.org/software/make)
  - [Pandoc 1.18.\*.\*](https://pandoc.org)
  - [pandoc-citeproc 0.10.\*.\*](https://github.com/jgm/pandoc-citeproc) (if you don’t use citations, you can just symlink this to `cat`)
  - [Rust 1.17.\*](https://www.rust-lang.org)
  - For PDF output:
      - [LaTeX 2e](https://www.latex-project.org)
        (including [latexmk 4.52c](https://www.ctan.org/pkg/latexmk))
      - If you have any SVG images:
          - [Inkscape 0.92.\*](https://inkscape.org/en/)

The version numbers are advisory: things will probably still work on something a bit older or newer.

The book generator also relies on a *patched* version of [mdBook](https://github.com/azerupi/mdBook) (see `mdbook.patch` in the source tree), but this will be automatically downloaded and installed to `./.local/bin/mdbook` by the build system.  The user shouldn't have to worry about this detail except on airgapped systems.
