#include <stdint.h>
#include <avr/interrupt.h>
#include <util/delay.h>
#include "serial.h"
#include "print_num.h"

#define ENVIA_BIN

#include "sinus.h"
#define SINUS hz_127

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
    uint8_t intents = 5;
    while(1) {
#ifdef ENVIA_BIN
        if (--index != 327) {
            serial_envia_byte(SINUS[index]);
            if (!index) {
                index = sizeof(SINUS);
            }
        }
        else if (!(--intents)) {
            serial_envia_byte('1');
            serial_envia_byte('4');
            serial_envia_byte('6');
            serial_envia_byte('\n');
            intents = 5;
        }
        else {
            serial_envia_byte(SINUS[index]);
        }
#else
        if (--index != 327) {
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
            intents = 5;
        }
        else {
            print_num_dec(SINUS[index]);
        }
#endif
        _delay_us(uTS);
    }
}