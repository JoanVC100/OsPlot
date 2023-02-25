#include <avr/io.h>
#include <stdint.h>
#include "adc.h"
#include "serial.h"

void adc_inicia(adc_pin_t pin, adc_vref_t vref, adc_prescaler_t prescaler) {
  ADMUX |= vref | pin | 1 << ADLAR;
  ADCSRB = 0; // Deshabilita el 'trigger' automàtic
  DIDR0 = 255; // Deshabilita tots els bufers digitals dels pins analògics.
  PORTC = 0; // Deshabilita pull-ups als pins analògics
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