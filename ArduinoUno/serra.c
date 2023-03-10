#include <stdint.h>
#include <avr/interrupt.h>
#include <util/delay.h>
#include "serial.h"
#include "print_num.h"

#define uTS 20
#define VALOR_MAXIM 255

int main() {
    serial_obre();
    sei();

    print_num_dec6(1000000/uTS);
    uint8_t lectura = VALOR_MAXIM;
    while(1) {
        print_num_dec(lectura);
        if (!lectura--) {
            lectura = VALOR_MAXIM;
        }
        _delay_us(uTS);
    }
}