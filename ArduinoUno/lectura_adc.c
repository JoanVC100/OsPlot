#include <stdint.h>
#include <avr/interrupt.h>
#include <util/delay.h>
#include "serial.h"
#include "adc.h"
#include "print_num.h"

#define PRESCALER p128

int main() {
    adc_inicia(a5, v5, PRESCALER);
    adc_inici_lectura();
    serial_obre();
    sei();

    print_num_dec6(ADC_CALCULA_FS(PRESCALER));
    while(1) {
        uint8_t lectura = adc_llegeix8();
        adc_inici_lectura();
        print_num_dec(lectura);
    }
}