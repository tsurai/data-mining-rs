set term svg size 800,600
set title "k-means - iris data set" font ",20"
set view ,110,
set style line 1 lc rgb "red"
set style line 2 lc rgb "blue"
set style line 3 lc rgb "green"
set output "kmeans.svg"

splot '../output/kmeans.data' i 0 u 1:2:3:4 w points lc var pt 2,\
      '../output/kmeans.data' i 1 u 1:2:3:4 w points lc var ps 2 pt 6
