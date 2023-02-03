#include "serial.h"
#include <avr/interrupt.h>

int main() {

    uint8_t n = 0;

    serial_obre();
    sei();

    while(1) {
        serial_envia(n++);
    }
}