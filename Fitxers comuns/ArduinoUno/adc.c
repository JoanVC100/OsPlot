#include <avr/io.h>
#include <stdint.h>
#include "adc.h"
#include "serial.h"
#include <util/delay.h>

void adc_inicia(adc_pin_t pin, adc_vref_t vref, adc_prescaler_t prescaler) {
  ADMUX |= vref | pin | 1 << ADLAR;
  DIDR0 = 0b00111111; // Deshabilita tots els bufers digitals dels pins analògics.
  switch (prescaler) {
    case p2: prescaler = 1; break;
    case p4: prescaler = 2; break;
    case p8: prescaler = 3; break;
    case p16: prescaler = 4; break;
    case p32: prescaler = 5; break;
    case p64: prescaler = 6; break;
    default: prescaler = 7; break;
  }
  ADCSRA |= (1 << ADEN) | prescaler;
}

void adc_inici_lectura(void) {
  ADCSRA |= 1 << ADSC;
}

uint8_t adc_llegeix8(void) {
  while (bit_is_set(ADCSRA, ADSC));
  return ADCH;
}

int adc_llegeix10(void) {
  while (bit_is_set(ADCSRA, ADSC));
  return (ADCL >> 6) + (ADCH << 2);
}

void adc_atura(void) {
  ADCSRA = 0;
  //ADCSRB = 0;
  ADMUX = 0;
  DIDR0 = 0;
}