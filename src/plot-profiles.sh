#!/bin/bash

for file in $(find ./movies-profiles -type f ! -name "*.*"); do
	echo $file;
	tit=$(echo $file|cut -d"/" -f3);
	echo $tit;
	cat $file|./src/dist > $file.dist
	cat $file.dist|./src/ccdf > $file.ccdf

	# Plot des ccdfs
	gnuplot -e "path='$file';tit='$tit';xlab='';ylab='';" ./src/plot.gp
	
done 

# Faire pdf
gs -sDEVICE=pdfwrite -dNOPAUSE -dBATCH -dSAFER -sOutputFile=./movies-profiles.pdf ./movies-profiles/*.eps

for file in $(find ./users-profiles -type f ! -name "*.*"); do
	echo $file;
	tit=$(echo $file|cut -d"/" -f3);
	echo $tit;
	cat $file|./src/dist > $file.dist
	cat $file.dist|./src/ccdf > $file.ccdf

	# Plot des ccdfs
	gnuplot -e "path='$file';tit='$tit';xlab='';ylab='';" ./src/plot.gp
	
done 

# Faire pdf
gs -sDEVICE=pdfwrite -dNOPAUSE -dBATCH -dSAFER -sOutputFile=./users-profiles.pdf ./users-profiles/*.eps