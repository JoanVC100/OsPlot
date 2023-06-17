#include <stdint.h>
#include <avr/interrupt.h>
#include <util/delay.h>
#include "serial.h"
#include "adc.h"
#include "print_num.h"

#define PRESCALER_ADC p128

int main() {
    adc_inicia(a5, v5, PRESCALER_ADC);
    adc_inici_lectura();
    serial_obre();
    sei();

    while(1) {
        uint8_t lectura = adc_llegeix8();
        adc_inici_lectura();
        serial_envia_byte(lectura);
    }
}