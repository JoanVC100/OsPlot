#include <stdint.h>
#include <avr/interrupt.h>
#include <util/delay.h>
#include "serial.h"
#include "print_num.h"

#include "sinus.h"
#define SINUS hz_127

int main() {
    serial_obre();
    sei();

    print_num_dec6(1000000/uTS);
    uint16_t index = sizeof(SINUS);
    uint8_t intents = 5;
    while(1) {
        if (--index != 327) {
            print_num_dec(SINUS[index]);
            if (!index) {
                index = sizeof(SINUS);
            }
        }
        else if (!(--intents)) {
            serial_envia('1');
            serial_envia('4');
            serial_envia('6');
            serial_envia('\n');
            intents = 5;
        }
        else {
            print_num_dec(SINUS[index]);
        }
        _delay_us(uTS);
    }
}