set term svg enhanced size 800,600
set title "k-medoids - iris data set" font ",20"
set style line 1 lc rgb "red"
set style line 2 lc rgb "blue"
set style line 3 lc rgb "green"
set style line 4 lc rgb "yellow" pt 7 
splot 'output.data' u 1:2:3:4 lc variable
