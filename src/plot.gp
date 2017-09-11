# Standard plot file (nothing fancy)
set title tit;

set ter pos col;
set out path.".eps";

set xlabel xlab font ",24";
set ylabel ylab font ",24";

plot path.".ccdf" notitle;
