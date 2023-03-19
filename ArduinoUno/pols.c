#include <avr/interrupt.h>
#include <util/delay.h>
#include "serial.h"
#include "print_num.h"

#define ENVIA_BIN

#define uTS 20
#define uTsenyal 40

#define FS 1000000/uTS

int main() {
    serial_obre();
    sei();

#ifdef ENVIA_BIN
    serial_llegir();
    serial_envia_4byte(FS);
#else
    print_num_dec6(FS);
#endif

    unsigned int n = uTsenyal/2;
    while(1) {
#ifdef ENVIA_BIN
        if (n >= uTsenyal/4) serial_envia_byte(255);
        else serial_envia_byte(0);
#else
        if (n >= uTsenyal/4) {
            serial_envia_byte('2');
            serial_envia_byte('5');
            serial_envia_byte('5');
        }
        else {
            serial_envia_byte('0');
            serial_envia_byte('0');
            serial_envia_byte('0');
        }
        serial_envia_byte('\n');
#endif
        if (!(--n)) {
            n = uTsenyal/2;
        }
        _delay_us(uTS);
    }
}