nivell_trigger=$(echo "${args[nivell]}" | bc 2> /dev/null)
directori_script="/tmp/OsPlot"

if [ -d $directori_script  ]; then
    if [[ $nivell_trigger == "${args[nivell]}" ]] && [[ $nivell_trigger != "$(echo -e "(standard_in) 1: syntax error\n")" ]]; then
        if [[ ! ${args[--sense-escalar]} ]]; then
            printf -v nivell_trigger %.0f $(echo "$nivell_trigger*255/5" | bc)
            if [[ $nivell_trigger -lt 0 ]] || [[ $nivell_trigger -gt 255 ]]; then
                echo "Rang de trigger incorrecte. El nivell ha d'estar entre 0 i 5"
                exit
            fi
        else
            LC_NUMERIC="en_US.UTF-8" printf -v nivell_trigger %.0f $nivell_trigger
            if [[ $nivell_trigger -lt 0 ]] || [[ $nivell_trigger -gt 255 ]]; then
                echo "Rang de trigger incorrecte. El nivell ha d'estar entre 0 i 255"
                exit
            fi
        fi
        echo $nivell_trigger > "$directori_script/nivell_trigger"
    else
        echo "El nivell del trigger no és un número, o està mal escrit"
    fi
else
    echo "No s'ha iniciat OsPlot"
fi