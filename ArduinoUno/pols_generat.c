#include <stdint.h>
#include <avr/interrupt.h>
#include <util/delay.h>
#include "serial.h"
#include "adc.h"
#include "print_num.h"

/* S'ha de connectar el pin 9 de l'Arduino a l'A5 */

//#define DEBUG 1

#define PRESCALER 64UL
#define FREQ_POLS 500UL

#define VALOR_OCR1A (F_CPU / (2UL*PRESCALER*FREQ_POLS))
#if VALOR_OCR1A == 0
    #error "Freqüència o prescaler massa grans"
#elif VALOR_OCR1A > 65535
    #error "Freqüència o prescaler massa petits"
#endif

int main() {
    ///////////// Timer
    TCCR1A |= 1 << COM1A0;
    OCR1A = (uint16_t) VALOR_OCR1A;
    DDRB |= 1 << PINB1;
    uint8_t prescaler;
    switch (PRESCALER) {
        case 1024: prescaler = 0b101; break;
        case 256: prescaler = 0b100; break;
        case 64: prescaler = 0b011; break;
        case 8: prescaler = 0b010; break;
        default: prescaler = 0b001; break;
    }
    TCCR1B |= (1 << WGM12) | prescaler;
    ////////////////////////////////////////////

    adc_inicia(a5, v5, p128);
    adc_inici_lectura();
    serial_obre();
    sei();

#ifdef DEBUG
    DDRD |= 1 << PIN7;
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
        print_num_dec(lectura);
    }
}