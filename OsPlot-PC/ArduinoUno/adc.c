#include <avr/io.h>
#include <stdint.h>
#include "adc.h"
#include "serial.h"
#include <util/delay.h>

void adc_inicia(adc_pin_t pin, adc_vref_t vref, adc_prescaler_t prescaler) {
  ADMUX |= vref | pin | 1 << ADLAR;
  ADCSRB = 0; // Deshabilita el 'trigger' automàtic
  DIDR0 = 255; // Deshabilita tots els bufers digitals dels pins analògics.
  PORTC = 0; // Deshabilita pull-ups als pins analògics
  switch (prescaler) {
    case p2: prescaler = 1;
    case p4: prescaler = 2;
    case p8: prescaler = 3;
    case p16: prescaler = 4;
    case p32: prescaler = 5;
    case p64: prescaler = 6;
    default: prescaler = 7;
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