#include <stdio.h>
#include <string.h>
#include <avr/interrupt.h>
#include <util/delay.h>
#include "serial.h"

#define uTS 20

int main() {

    uint8_t lectura = 255;

    serial_obre();
    sei();

    char s[4] = {0,0,0,0};
    while(1) {
        snprintf(s, sizeof(s), "%03d", lectura);
        for (uint8_t c = 0; s[c] != '\0'; c++) {
            serial_envia(s[c]);
        }
        serial_envia('\n');
        if (!lectura--) {
            lectura = 255;
        }
        _delay_us(uTS);
    }
}