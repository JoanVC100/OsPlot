#include <avr/interrupt.h>
#include <util/delay.h>
#include "serial.h"
#include "print_num.h"

#define uTS 20
#define uTsenyal 40

int main() {
    serial_obre();
    sei();

    print_num_dec6(1000000/uTS);
    unsigned int n = uTsenyal/2;
    while(1) {
        if (n >= uTsenyal/4) {
            serial_envia('2');
            serial_envia('5');
            serial_envia('5');
        }
        else {
            serial_envia('0');
            serial_envia('0');
            serial_envia('0');
        }
        serial_envia('\n');
        if (!(--n)) {
            n = uTsenyal/2;
        }
        _delay_us(uTS);
    }
}