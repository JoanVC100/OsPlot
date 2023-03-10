#include <stdint.h>
#include <avr/interrupt.h>
#include <util/delay.h>
#include "serial.h"
#include "print_num.h"

#include "sinus.h"
#define SINUS hz_100

int main() {
    serial_obre();
    sei();

    print_num_dec6(1000000/uTS);
    uint16_t index = sizeof(SINUS)-1;
    while(1) {
        print_num_dec(SINUS[index]);
        if (!index--) {
            index = sizeof(SINUS)-1;
        }
        _delay_us(uTS);
    }
}