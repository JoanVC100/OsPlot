#include <avr/interrupt.h>
#include <util/delay.h>
#include "serial.h"
#include "print_num.h"

#define ENVIA_BIN

#define uTS 20
#define uTsenyal 10000

const float fs = 1000000/uTS;

#define N_SENYAL uTsenyal/uTS

int main() {
    serial_obre();
    sei();

#ifdef ENVIA_BIN
    serial_llegir_byte();
    serial_envia_4byte((uint8_t*) &fs);
#else
    print_num_dec6(FS);
#endif

    unsigned int n = N_SENYAL;
    while(1) {
#ifdef ENVIA_BIN
        if (n >= N_SENYAL/2) serial_envia_byte(255);
        else serial_envia_byte(0);
#else
        if (n >= N_SENYAL/2) {
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
            n = N_SENYAL;
        }
        _delay_us(uTS);
    }
}