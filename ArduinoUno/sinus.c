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
    serial_llegir();
    serial_envia_4byte(SINUS_FS);
#else
    print_num_dec6(Fs);
#endif

    uint16_t index = sizeof(SINUS)-1;
    while(1) {
#ifdef ENVIA_BIN
        uint32_t a = SINUS[index];
        serial_envia_byte(a);
#else
        print_num_dec(SINUS[index]);
#endif
        if (!index--) {
            index = sizeof(SINUS)-1;
        }
        _delay_us(uTS);
    }
}