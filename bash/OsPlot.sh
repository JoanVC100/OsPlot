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
    captura=0 # 0 si no es pot capturar, 1 si sí es pot
    nivell_trigger=1
    n1_lectura=0 # Lectura anterior
    en_pujada=1 # Flanc descendent sii és 0. Si hi ha flanc
    #ascendent i la lectura esta per sota el trigger, és 1
    while read -r linia; do
        n_lectura=$((10#$linia))
        deriv=$(($n_lectura-$n1_lectura))
        n1_lectura=$n_lectura
        if (( $deriv > 0 && $n_lectura <= $nivell_trigger )); then
            en_pujada=1
        elif (( $deriv < 0 )); then
            en_pujada=0
        fi
        if (( $captura == 1 )); then
            printf -v s "%s%s\n" "$s" "$n_lectura"
            ((n=n-1))
            if (( !n )); then
                echo "$s" > $cua_dades
                n=$n_mostres
                s=""
                captura=0
                en_pujada=0
            fi
        else
            if (( $en_pujada == 1 && $n_lectura >= $nivell_trigger )); then
                captura=1
            fi
        fi
    done < $port
}

bucle_lectura &
pid_bucle=$!
gnuplot -e "cua_lectura='${cua_dades}'; fs=${fs}" ${directori}/plot.gnu
pid_plot=$!
sleep infinity






