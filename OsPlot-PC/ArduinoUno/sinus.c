#include <stdint.h>
#include <avr/interrupt.h>
#include <util/delay.h>
#include "serial.h"
#include "print_num.h"

#include "sinus.h"
#define SINUS hz_100

#define ENVIA_BIN

int main() {
    serial_obre();
    sei();

#ifdef ENVIA_BIN
    serial_llegir_byte();
    serial_envia_4byte((uint8_t*) &fs);
#else
    print_num_dec6(SINUS_FS);
#endif

    uint16_t index = sizeof(SINUS)-1;
    while(1) {
#ifdef ENVIA_BIN
        serial_envia_byte(SINUS[index]);
#else
        print_num_dec(SINUS[index]);
#endif
        if (!index--) {
            index = sizeof(SINUS)-1;
        }
        _delay_us(uTS);
    }
}