#!/bin/bash
cua_dades="/tmp/osplot.dat"
directori=$(dirname $(realpath -s "$0"))
fs=50e3

function surt {
    rm -f $cua_dades
    kill $pid_bucle
    kill $pid_plot
    exit
}
trap surt SIGINT EXIT

port=$1
if [[ ! $port ]]; then
    echo "No s'ha donat un port sèrie"
    exit
fi
if [[ ! -e $port ]]; then
    echo "El port sèrie donat no existeix"
    exit
fi
rm -f $cua_dades
mkfifo $cua_dades

n_mostres=500
function bucle_lectura {
    stty -F $port sane
    stty -F $port raw speed 1000000 -echo > /dev/null # Necessari perque cat o read no es pengin
    n=$n_mostres
    s=""
    while read -r linia; do
        printf -v s "%s%s\n" "$s" "$linia"
        ((n=n-1))
        if (( !n )); then
            echo "$s" > $cua_dades
            n=$n_mostres
            s=""
        fi
    done < $port
}

bucle_lectura &
pid_bucle=$!
gnuplot -e "cua_lectura='${cua_dades}'; fs=${fs}" ${directori}/plot.gnu
pid_plot=$!
sleep infinity






