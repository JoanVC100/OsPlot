#include <stdint.h>
#include <avr/interrupt.h>
#include <util/delay.h>
#include "serial.h"
#include "adc.h"
#include "print_num.h"

int main() {
    adc_inicia(a5, v5, p128);
    adc_inici_lectura();
    serial_obre();
    sei();

    while(1) {
        uint8_t lectura = adc_llegeix8();
        adc_inici_lectura();
        print_num_dec(lectura);
    }
}