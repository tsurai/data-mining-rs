set term svg size 800,600
set title "dbscan - cassini data set" font ",20"
set style line 1 lc rgb "red"
set style line 2 lc rgb "blue"
set style line 3 lc rgb "green"
set output "dbscan.svg"

plot '../output/dbscan.data' u 1:2:3 w points lc var pt 2
