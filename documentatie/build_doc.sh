[ ! -d "doc/" ] && mkdir doc

for mdfile in *.md ; do
    #pandoc $mdfile --pdf-engine=pdflatex --toc -N -o build/$mdfile.pdf
    pdffile=`echo $mdfile | cut -f 1 -d '.'`;
    pandoc $mdfile --pdf-engine=pdflatex -o doc/`echo $pdffile | cut -f 1 -d '.'`.pdf;
done