#!/bin/bash
cua_dades="/tmp/osplot.dat"
directori=$(dirname $(realpath -s "$0"))
fs=$((16000000/128/13))
n_mostres=500
nivell_trigger=128
debug="0" # "1" per habilitar, "0" per deshabilitar

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

function bucle_lectura {
    stty -F $port sane
    stty -F $port raw speed 1000000 -echo > /dev/null # Necessari perque cat o read no es pengin
    n_mostres_llegides=$n_mostres
    mostres_llegides=""
    n1_lectura=0
    e_inicial=0; e_esperant_trigger=1; e_capturant=2;
    estat=$e_inicial
    while read -r linia; do
        n_lectura=$((10#$linia))
        case $estat in
            $e_esperant_trigger)
                if (( ($n1_lectura <= $nivell_trigger && $n_lectura >= $nivell_trigger) )); then
                    estat=$e_capturant
                    echo "$n_lectura $n1_lectura"
                fi
            ;;
            $e_capturant)
                printf -v mostres_llegides "%s%s\n" "$mostres_llegides" "$n_lectura"
                ((n_mostres_llegides=n_mostres_llegides-1))
                if (( !n_mostres_llegides )); then
                    echo "$mostres_llegides" > $cua_dades
                    n_mostres_llegides=$n_mostres
                    mostres_llegides=""
                    estat=$e_esperant_trigger;
                    echo "DONE"
                fi
            ;;
            $e_inicial)
                estat=$e_esperant_trigger
            ;;
        esac
        n1_lectura=$n_lectura
    done < $port
}

bucle_lectura &
pid_bucle=$!
if [[ $debug -eq "1" ]]; then
    gnuplot -e "cua_lectura='${cua_dades}'; fs=${fs}" ${directori}/plot_debug.gnu
else
    gnuplot -e "cua_lectura='${cua_dades}'; fs=${fs}" ${directori}/plot.gnu
fi
pid_plot=$!
sleep infinity






