set style line 1 \
    linecolor rgb "#0060ad" \
    linetype 1 linewidth 2 \
    pointtype 7 pointsize 0

set yrange [0:5]
plot cua_lectura using (column(0)/fs):(column(1)*5/256) with linespoints linestyle 1
while (1) {
    replot
}