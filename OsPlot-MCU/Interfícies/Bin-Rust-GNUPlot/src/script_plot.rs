macro_rules! script_plot {
    () => {
        concat!(
            "set style line 1 \\\n",
            "    linecolor rgb \"#0060ad\" \\\n",
            "    linetype 1 linewidth 2 \\\n",
            "    pointtype 7 pointsize 0\n",
            "set yrange [0:5]\n",
            "plot cua binary format=\"%float32%uchar\" using (column(1)):(column(2)*5/255) with linespoints linestyle 1\n",
            "while (1) {replot}\n"
        )
    };
}