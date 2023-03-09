set style line 1 \
    linecolor rgb "#0060ad" \
    linetype 1 linewidth 2 \
    pointtype 7 pointsize 0

set yrange [0:255]
plot cua_lectura using (column(0)/fs):(column(1)) with linespoints linestyle 1
while (1) {
    replot
}