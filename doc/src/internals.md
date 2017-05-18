    src -------> target/stage ------> target/<format>
        frontend              backend

  - Frontend: format-independent processing through Pandoc
  - Backend: format-specific rendering through Pandoc and mdBook/LaTeX

## Frontend

    src/*.md

        |
        | prepend-heading
        v

    [md]

        |
        | pandoc (frontend)
        v

    [json]

        |
        | preprocessor
        v

    target/stage/src/*.json

Each Markdown file is transformed individually into an output-independent JSON file using Pandoc.  The use of JSON ensures that all the details are preserved correctly, including citations (which would get lost if HTML was used for intermediate files).  The preprocessor is not yet implemented.

The transformation of each file is independent of each other; there is no state maintained.

Still, even though the transformations are totally independent, the makefile has to know which files need to be transformed.  This is what `target/stage/src/SUMMARY.mk` stores: a list of the book items in the correct ordering as a makefile variable `$(item_names)`.  It is obtained via `.local/bin/get-book-items`.

## Backend

Using the files in the staging directory (`target/stage`), we use mdBook to parse the structure of the book items (chapters, sections, etc) in `SUMMARY.md` and render the output files.  This is handled by the following custom Rust tools:

  - `latex-merge`, which reads `SUMMARY.md` and combines the various `target/stage/src/*.json` files into a single JSON file `target/pdf/book.json` to be consumed by Pandoc for PDF generation.
  - `append-biblio-title`, which, if `-M biblio-title=<title>` is set, appends the title of the bibliography to each chapter of the HTML book just before `pandoc-citeproc` is run.
  - `html-mdbook-toml`: which translates the Makefile variable `$(metadata)` into `target/stage/html/book.toml` for mdBook.

### HTML Output

    target/stage/src/*.json

            |
            | append-biblio-title
            v

    [json]

            |
            | pandoc-citeproc
            v

    [json]

            |
            | pandoc (HTML)
            v

    target/stage/html/src/*.htm

            |
            | mdbook
            v

    target/html

The JSON files are translated into `.htm` using Pandoc.  The choice of `.htm` instead of `.html` is mainly to work around an mdBook quirk.

In principle, this one should be straightforward: just call `mdbook build <dir>`.  In practice, we have to use a patched version of mdBook because we want to render Markdown using Pandoc rather than the default pulldown-cmark.

#### The mdBook patch

We use a patched version of mdBook that renders directly through HTML rather than through Markdown.  This is because we use want to use Pandoc to preprocess the input files and “standard” Markdown is not very expressive.

Originally, the plan was to preprocess with Pandoc and save them as CommonMark so that mdBook could parse them.  However, the problem is that (a) Pandoc does not support a lossless conversion to CommonMark (b) pulldown-cmark does not adhere to CommonMark very well (e.g. it does not preserve whitespace within `<pre>` elements as the spec demands).

Therefore, we simply disable the Markdown processing altogether and use HTML.  To avoid duplicating a large chunk of code from mdBook, so we opted to just patch the original code instead.

#### Using `.htm` instead of `.html`

After rendering, mdBook copies all files except `.md` files into the output directory.  If we named the input files `.html` they would immediately overwrite the files we just generated!  Moreover, using `.htm` makes it easy to delete the input files from the output directory afterward.

#### Keeping `target/html` tidy

Part of the reason to have `target/stage` is so that we can select precisely what files we want.  mdBook is not very selective so it will copy everything in `src` into `dest` and then purge all the `.md` files.

### PDF Output

    target/stage/src/*.json

            |
            | latex-merge
            v

    target/pdf/book{_head.tex,{_before,,_after}.json}

            |
            | pandoc (LaTeX)
            v

    target/pdf/book.tex

            |
            | latexmk
            v

    target/pdf/book.pdf

`latex-merge` is responsible for combining the chapters together into a single JSON file, shifting the heading levels as necessary.  Then `Pandoc` is invoked to generate the `.tex` file.  Finally we run `latexmk` to produce the PDF.

The main reason for invoking `Pandoc` within `latexbook` is because there are auxiliary files (e.g. frontmatter file) that would otherwise get entangled with the Makefile.

#### Location of the `.bib` file

It’s much easier to have the `.bib` file at the top-level.  The reason is that we want it to work with both `pandoc-citeproc`, which treats it as a path relative to the top-level directory, and `natbib`, which treats it as a path relative to `target/pdf`.  If we put it inside `src`, then `pandoc-citeproc` would complain, and mdbook would unwittingly copy the `.bib` file into `target/html`.
