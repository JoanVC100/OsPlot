#ifndef _ADC_
#define _ADC_

#include <stdint.h>

typedef enum {a0=0, a1, a2, a3, a4, a5} adc_pin_t;
typedef enum {v5=0b01000000, vAREF=0b00000000, v1_1=0b11000000} adc_vref_t;
typedef enum {p2=1, p4, p8, p16, p32, p64, p128} adc_prescaler_t;

void adc_inicia(adc_pin_t pin, adc_vref_t vref, adc_prescaler_t prescaler);
/* Inicia l'ADC en mode manual. Té com arguments el pin triat,
   la tensió de referència i el 'prescaler' a fer servir. */

void adc_inici_lectura(void);
// Inicia una lectura de l'ADC.

uint8_t adc_llegeix8(void);
/* Retorna els 8 bits de més pes de la lectura de l'ADC.
   Requereix que s'hagi iniciat una lectura. */

int adc_llegeix10(void);
/* Retorna la lectura de l'ADC. Requereix que s'hagi
   iniciat una lectura. */

#endif
