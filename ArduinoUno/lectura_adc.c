#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include <math.h>
#include <avr/interrupt.h>
#include <util/delay.h>
#include "serial.h"
#include "adc.h"

int main() {

    adc_inicia(a5, v5, p128);
    adc_inici_lectura();
    serial_obre();
    sei();

    char s[4] = {0, 0, 0, 0};
    while(1) {
        int lectura = adc_llegeix8();
        adc_inici_lectura();
        snprintf(s, sizeof(s), "%03d", lectura);
        for (uint8_t c = 0; s[c] != '\0'; c++) {
            serial_envia(s[c]);
        }
        serial_envia('\n');
    }
}