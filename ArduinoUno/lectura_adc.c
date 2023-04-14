#include <stdint.h>
#include <avr/interrupt.h>
#include <util/delay.h>
#include "serial.h"
#include "adc.h"
#include "print_num.h"

#define ENVIA_BIN
//#define DEBUG

#define PRESCALER_ADC p128

int main() {
    adc_inicia(a5, v5, PRESCALER_ADC);
    adc_inici_lectura();
    serial_obre();
    sei();

#ifdef DEBUG
    DDRD |= 1 << PIN7;
#endif

#ifdef ENVIA_BIN
    serial_llegir();
    serial_envia_4byte(ADC_CALCULA_FS(PRESCALER_ADC));
#else
    print_num_dec6(ADC_CALCULA_FS(PRESCALER_ADC));
#endif

    while(1) {
#ifdef DEBUG
        PORTD = 1 << PIN7;
#endif
        uint8_t lectura = adc_llegeix8();
#ifdef DEBUG
        PORTD = 0;
#endif
        adc_inici_lectura();
    
#ifdef ENVIA_BIN
        serial_envia_byte(lectura);
#else
        print_num_dec(lectura);
#endif
    }
}