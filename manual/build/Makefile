.PHONY: pdf
pdf: clean
	inkscape -D schema.svg -o schema.pdf
	pdflatex manual.tex
	pdflatex manual.tex

.PHONY: clean
clean:
	rm -rf *.pdf *.log *.aux .auctex-auto
