.PHONY: pdf
pdf: clean
	inkscape -D schema.svg -o schema.pdf --export-latex
	sed -i 's/% \(.*\ % paper leaflet format\)/\1/' manual.tex
	sed -i 's/\(.*\ % digital leaflet format\)/% \1/' manual.tex
	pdflatex -jobname=manual_paper manual.tex
	sed -i 's/\(.*\ % paper leaflet format\)/% \1/' manual.tex
	sed -i 's/% \(.*\ % digital leaflet format\)/\1/' manual.tex
	pdflatex -jobname=manual_digital manual.tex

.PHONY: clean
clean:
	rm -rf *.pdf *.pdf_tex *.log *.aux .auctex-auto
