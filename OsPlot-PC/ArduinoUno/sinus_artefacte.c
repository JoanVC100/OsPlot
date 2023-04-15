#include <stdint.h>
#include <avr/interrupt.h>
#include <util/delay.h>
#include "serial.h"
#include "print_num.h"

#define ENVIA_BIN

#include "sinus.h"
#define SINUS hz_100

#define INTENTS_ARTEFACTE 5
#define INDEX_ARTEFACTE 327

int main() {
    serial_obre();
    sei();

#ifdef ENVIA_BIN
    serial_llegir();
    serial_envia_4byte(SINUS_FS);
#else
    print_num_dec6(SINUS_FS);
#endif

    uint16_t index = sizeof(SINUS);
    uint8_t intents = INTENTS_ARTEFACTE;
    while(1) {
#ifdef ENVIA_BIN
        if (--index != INDEX_ARTEFACTE) {
            serial_envia_byte(SINUS[index]);
            if (!index) {
                index = sizeof(SINUS);
            }
        }
        else if (!(--intents)) {
            serial_envia_byte(146);
            intents = INTENTS_ARTEFACTE;
        }
        else {
            serial_envia_byte(SINUS[index]);
        }
#else
        if (--index != INDEX_ARTEFACTE) {
            print_num_dec(SINUS[index]);
            if (!index) {
                index = sizeof(SINUS);
            }
        }
        else if (!(--intents)) {
            serial_envia_byte('1');
            serial_envia_byte('4');
            serial_envia_byte('6');
            serial_envia_byte('\n');
            intents = INTENTS_ARTEFACTE;
        }
        else {
            print_num_dec(SINUS[index]);
        }
#endif
        _delay_us(uTS);
    }
}