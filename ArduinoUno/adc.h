#ifndef _ADC_H_
#define _ADC_H_

#include <stdint.h>

typedef enum {a0=0, a1, a2, a3, a4, a5} adc_pin_t;
typedef enum {v5=0b01000000, vAREF=0b00000000, v1_1=0b11000000} adc_vref_t;
typedef enum {p2=2, p4=4, p8=8, p16=16, p32=32, p64=64, p128=128} adc_prescaler_t;

#define ADC_CALCULA_FS(PRESCALER) (F_CPU/PRESCALER/13)

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
