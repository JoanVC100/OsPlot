#!/bin/bash
directori_treball="/tmp/OsPlot"
defecte_n_mostres=500 # 500 mostres per defecte
defecte_nivell_trigger=128 # 128 per defecte
debug="0" # "1" per habilitar, "0" per deshabilitar

cua_dades="$directori_treball/osplot.dat"
directori_script=$(dirname $(realpath -s "$0"))
conf_n_mostres="$directori_treball/mostres_finestra"
conf_nivell_trigger="$directori_treball/nivell_trigger"

function surt {
    stty -F $port sane
    rm -rf $directori_treball
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
rm -rf $directori_treball
mkdir $directori_treball
mkfifo $cua_dades
echo $defecte_n_mostres > $conf_n_mostres
echo $defecte_nivell_trigger > $conf_nivell_trigger

function actualitza_configuracio {
    n_mostres_llegides=$(sed "1q;d" $conf_n_mostres)
    if (( $n_mostres_llegides < 50 || $n_mostres_llegides > 1000 )); then
        n_mostres_llegides=$defecte_n_mostres
        echo $defecte_n_mostres > $conf_n_mostres
    fi
    nivell_trigger=$(sed "1q;d" $conf_nivell_trigger)
    if (( $nivell_trigger < 0 || $nivell_trigger > 255 )); then
        nivell_trigger=$defecte_nivell_trigger
        echo $defecte_nivell_trigger > $conf_nivell_trigger
    fi
}
function bucle_lectura {
    n_mostres_llegides=$defecte_n_mostres
    nivell_trigger=$defecte_nivell_trigger
    mostres_llegides=""
    n1_lectura=0
    e_inicial=0; e_esperant_trigger=1; e_capturant=2;
    estat=$e_inicial
    stty -F $port sane
    stty -F $port raw speed 1000000 -echo > /dev/null # "-echo" necessari perque cat o read no es pengin
    while read -r linia; do
        n_lectura=$((10#$linia))
        case $estat in
            $e_esperant_trigger)
                if (( ($n1_lectura <= $nivell_trigger && $n_lectura >= $nivell_trigger) )); then
                    estat=$e_capturant
                fi
                n1_lectura=$n_lectura
            ;;
            $e_capturant)
                printf -v mostres_llegides "%s%s\n" "$mostres_llegides" "$n_lectura"
                ((n_mostres_llegides=n_mostres_llegides-1))
                if (( !n_mostres_llegides )); then
                    echo "$mostres_llegides" > $cua_dades
                    mostres_llegides=""
                    actualitza_configuracio
                    estat=$e_esperant_trigger
                fi
                n1_lectura=$n_lectura
            ;;
            $e_inicial)
                fs=$n_lectura # La primera lectura és la Fs.
                if [[ $debug -eq "1" ]]; then
                    gnuplot -e "cua_lectura='${cua_dades}'; fs=${fs}" ${directori_script}/plot_debug.gnu &
                else
                    gnuplot -e "cua_lectura='${cua_dades}'; fs=${fs}" ${directori_script}/plot.gnu &
                fi
                estat=$e_esperant_trigger
            ;;
        esac
    done < $port
}

bucle_lectura