CARGO?=cargo
INKSCAPE?=inkscape
LATEXMK?=latexmk
PANDOC?=pandoc
PANDOC_CITEPROC?=pandoc-citeproc
PANDOC_CROSSREF?=pandoc-crossref

latex_documentclass?=book
base_pandoc_args=--top-level-division=chapter -M documentclass=$(latex_documentclass) -M link-citations=true -M linkReferences=true

tool_dir?=.

all: target/html target/pdf

clean:
	rm -fr target

deploy-gh-pages: dist/.git/config all
	cp -R target/html/* $(<D)/..
	ln -f target/pdf/book.pdf $(<D)/..
	cd $(<D)/.. && \
	git add -A && \
	git commit --amend -q -m Autogenerated && \
	git push -f origin master:gh-pages

target/html: target/html/index.html

target/pdf: target/pdf/book.pdf

.PHONY: all clean deploy-gh-pages target/html target/pdf

# Dependencies
# ------------

# set $(item_names) to list of all src/*.md files with correct ordering; if
# the *command* of rule depends on $(item_names) or its dependents, it should
# include $(target/stage/src/SUMMARY.mk) as a prerequisite to ensure it gets rebuilt
# when this changes
-include target/stage/src/SUMMARY.mk

target/stage/src/SUMMARY.mk: src/SUMMARY.md .local/bin/get-book-items
	@mkdir -p $(@D)
	@echo 'item_names='`$(word 2,$^) $(<D)` >$@

items=$(addprefix src/,$(item_names))
json_items=$(patsubst %.md,target/stage/%.json,$(items))
assets+=$(shell find src -not -name '*.md' -type f)

# Deployment
# ----------

dist/.git/config:
	mkdir -p $(@D)
	url=`git remote -v | grep origin | awk '{ printf "%s", $$2; exit }'` && \
	cd $(@D)/.. && \
	git init && \
	git config user.name Bot && \
	git config user.email "<>" && \
	git commit -m _ --allow-empty && \
	git remote add origin "$$url"

# Staging
# -------

target/stage/src/SUMMARY.md: src/SUMMARY.md
	@mkdir -p $(@D)
	sed 's/\.md) *$$/\.json)/' $< >$@

target/stage/src/%.json: src/%.md $(tool_dir)/prepend-heading src/SUMMARY.md Makefile
	@mkdir -p $(@D)
	$(wordlist 2,3,$^) $< | $(PANDOC) --natbib $(base_pandoc_args) $(metadata) $(pandoc_args) $(pp_pandoc_args) -s -f markdown -o $@
# --natbib prevents pandoc from auto-invoking pandoc-citeproc

target/stage/src/%: src/%
	@mkdir -p $(@D)
	ln -f $< $@

# HTML Output
# -----------

# clean target/html to prevent deleted files from being deployed
target/html/index.html: target/stage/html/book.toml .local/bin/mdbook target/stage/html/book.ok $(addprefix target/stage/html/,$(assets))
	@dir='$(@D)' && rm -fr "$${dir}"
	$(word 2,$^) build --no-create $(<D)
	@( cd $(@D) && rm -f *.htm; )
	@touch $@

target/stage/html/book.toml: Makefile .local/bin/html-mdbook-toml
	@mkdir -p $(@D)
	$(word 2,$^) ../../html $(metadata) >$@

target/stage/html/book.ok: target/stage/src/SUMMARY.md .local/bin/html-merge .local/bin/html-fix-links .local/bin/html-split Makefile target/stage/html/src/SUMMARY.md $(json_items)
	@mkdir -p $(@D)/src
	$(word 2,$^) --biblio-path=bibliography --final-ext=html --output-ext=htm --output-dir=$(@D)/src $(<D) | $(PANDOC_CROSSREF) html | $(PANDOC_CITEPROC) html | $(word 3,$^) | pandoc $(base_pandoc_args) $(pandoc_args) $(html_pandoc_args) -f json -t html | $(word 4,$^) --output-ext=htm --output-dir=$(@D)/src
	@touch $@

target/stage/html/src/%: target/stage/src/%
	@mkdir -p $(@D)
	@ln -f $< $@

# PDF Output
# -----------

target/pdf/book.pdf: $(patsubst src/%,target/pdf/%,$(patsubst %.svg,%.pdf,$(assets)))

target/pdf/book.tex: .local/bin/latex-merge target/stage/src/SUMMARY.md $(latex_pandoc_deps) Makefile $(addprefix target/pdf/,$(wildcard *.bib)) $(json_items)
	@mkdir -p $(@D)
	$(wordlist 1,2,$^) $(latex_merge_args) $(basename $@)
	$(PANDOC_CROSSREF) latex <$(basename $@)_before.json | $(PANDOC) $(base_pandoc_args) $(pandoc_args) -f json -o $(basename $@)_before.tex
	$(PANDOC_CROSSREF) latex <$(basename $@)_after.json | $(PANDOC) $(base_pandoc_args) $(pandoc_args) -f json -o $(basename $@)_after.tex
	$(PANDOC_CROSSREF) latex <$(basename $@).json | $(PANDOC)  $(base_pandoc_args) -H $(basename $@)_head.tex -B $(basename $@)_before.tex -A $(basename $@)_after.tex $(pandoc_args) $(call latex_pandoc_args,$(latex_pandoc_deps)) -f json -o $@

target/pdf/%.bib: %.bib
	@mkdir -p $(@D)
	ln -f $< $@

target/pdf/%: target/stage/src/%
	@mkdir -p $(@D)
	ln -f $< $@

# Build $(tool_dir)
# -----------

# set PATH to avoid unnecessary warning
cargo_install=PATH=.local/bin:$$PATH $(CARGO) install -f --root .local

.local/bin/%: .local/tmp/tools.built
	@touch $@

.local/tmp/tools.built: $(tool_dir)/Cargo.toml $(shell find $(tool_dir)/src -type d -o -type f)
	$(cargo_install) --path $(<D)
	@mkdir -p $(@D)
	@touch $@

.local/bin/mdbook: .local/src/mdbook.tar.gz $(tool_dir)/mdbook.patch
	gunzip <$< | ( cd $(<D) && tar xf -; )
	( cd $(<D)/mdBook-* && patch -N -p 0 ) <$(word 2,$^)
	$(cargo_install) --path $(<D)/mdBook-*

.local/src/mdbook.tar.gz:
	@mkdir -p $(@D)
	$(tool_dir)/download https://github.com/azerupi/mdBook/archive/0.0.21.tar.gz $@

# Generic rules
# -------------

%.pdf: %.svg
	@mkdir -p $(@D)
	$(INKSCAPE) --without-gui --export-pdf=$@ $<

# latexmk can automatically mkdir if necessary
%.pdf: %.tex
	$(LATEXMK) -g -pdf -interaction=nonstopmode -cd $<

.DELETE_ON_ERROR:

.SECONDARY:
