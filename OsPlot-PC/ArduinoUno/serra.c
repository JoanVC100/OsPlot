#include <stdint.h>
#include <avr/interrupt.h>
#include <util/delay.h>
#include "serial.h"
#include "print_num.h"

#define ENVIA_BIN

#define uTS 20
#define VALOR_MAXIM 255

const float fs = 1000000/uTS;

int main() {
    serial_obre();
    sei();

#ifdef ENVIA_BIN
    serial_llegir_byte();
    serial_envia_4byte((uint8_t*) &fs);
#else
    print_num_dec6(FS);
#endif

    uint8_t lectura = VALOR_MAXIM;
    while(1) {
#ifdef ENVIA_BIN
        serial_envia_byte(lectura);
#else
        print_num_dec(lectura);
#endif
        if (!lectura--) {
            lectura = VALOR_MAXIM;
        }
        _delay_us(uTS);
    }
}