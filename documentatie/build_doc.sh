[ ! -d "doc/" ] && mkdir doc

for mdfile in *.md ; do
    #pandoc $mdfile --pdf-engine=pdflatex --toc -N -o build/$mdfile.pdf
    pandoc $mdfile -f markdown -t context --toc -V documentclass=report --from gfm -V geometry:margin=2cm -V fontsize:12pt -o doc/`echo $mdfile | cut -f 1 -d '.'`.pdf;
done

mv doc/README.pdf doc/pid.pdf