metadata=-M title="book-mk Documentation" \
         -M author="Phil Ruffwind"

pandoc_args=--number-sections --natbib \
            -M bibliography=book.bib \
            -M colorlinks \
            -M fontsize=12pt \
	    -M toc

latex_pandoc_args=-H head.tex

tool_dir=..
include $(tool_dir)/book.mk
