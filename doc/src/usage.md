## Minimal example

Start by creating an empty directory.  This will serve as the source tree of your book.

Download the `book-mk` source code and extract its contents into a subdirectory `./book-mk` so that "`./book-mk/book.mk`" points to the makefile.  Other than things like `./book-mk/README.md` or `./book-mk/doc.md`, most of the files under `./book-mk` are essential and should not be removed.

Create `./Makefile` with the following contents:

```make
metadata=-M title="My first book" \
         -M author="Your Name"

tool_dir=book-mk
include $(tool_dir)/book.mk
```

This is your top-level makefile and is the entry point of the build system.  (The `metadata` and `tool_dir` variables are special, so don't rename them!)

Next, create `./src/SUMMARY.md` with your table of contents:

```
[Preface](preface.md)
- [Chapter 1](chapter-1.md)
- [Chapter 2](chapter-2.md)
  - [Section 2.1](section-2-1.md)
[Epilogue](epilogue.md)
```

Finally, create a file `./src/preface.md` with the following contents:

    Hello world!

Don't worry about the other chapters -- the build system will automatically create them if they're missing.

## Building the book

Simply run:

    make target/html target/pdf

  - The HTML version is at `target/html/index.html`.
  - The PDF version is at `target/pdf/book.pdf`.  There'll be some auxiliary files created in `target/pdf` as well, but none of them are essential.  They can be useful for debugging problems though.

The makefile also provides the following commands (phony targets):

  - `clean`: Remove the `target` directory.  This does *not* remove the `.local` directory, however, because building those can often take some time.
  - `deploy-gh-pages`: Upload the HTML and PDF books to GitHub pages using the `origin` remote of the current Git repository.
