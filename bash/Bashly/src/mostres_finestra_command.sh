n_mostres=$(echo "${args[n_mostres]}" | bc 2> /dev/null)
directori_script="/tmp/OsPlot"

if [ -d $directori_script  ]; then
    if [[ $n_mostres == "${args[n_mostres]}" ]] && [[ $n_mostres != "$(echo -e "(standard_in) 1: syntax error\n")" ]]; then
        if [[ $n_mostres -lt 50 ]] || [[ $n_mostres -gt 1000 ]]; then
            echo "Número de mostres invàl·lid. El número ha d'estar entre 50 i 1000".
        else
            echo $n_mostres > $directori_script/mostres_finestra
        fi
    else
        echo "El número de mostres no és un número, o està mal escrit"
    fi
else
    echo "No s'ha iniciat OsPlot"
fi