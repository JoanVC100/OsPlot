#include <stdint.h>
#include <avr/interrupt.h>
#include <util/delay.h>
#include "serial.h"
#include "adc.h"
#include "print_num.h"

#define PRESCALER_ADC p128

#define BYTE_ESCAPAMENT 128
#define DEFECTE_NIVELL_TRIGGER 128
typedef enum {e_esperant_trigger, e_capturant} estats_trigger_t;
volatile estats_trigger_t estat_trigger = e_esperant_trigger;
const uint8_t nivell_trigger = DEFECTE_NIVELL_TRIGGER;
const uint16_t n_mostres_finestra = 500;

int main() {
    adc_inicia(a5, v5, PRESCALER_ADC);
    serial_obre();
    sei();

    adc_inici_lectura();
    uint8_t n_lectura, n1_lectura = adc_llegeix8();
    uint16_t n_mostres_finestra_actual = n_mostres_finestra;
    adc_inici_lectura();
    while(1) {
        n_lectura = adc_llegeix8();
        adc_inici_lectura();
        switch (estat_trigger) {
        case e_esperant_trigger:
            if (n1_lectura <= nivell_trigger && n_lectura >= nivell_trigger) {
                serial_envia_byte(n_lectura);
                if (n_lectura == BYTE_ESCAPAMENT)
                    serial_envia_byte(BYTE_ESCAPAMENT);
                n_mostres_finestra_actual--;
                estat_trigger = e_capturant;
            }                    
            break;
        case e_capturant:
            serial_envia_byte(n_lectura);
            if (n_lectura == BYTE_ESCAPAMENT)
                serial_envia_byte(BYTE_ESCAPAMENT);
            if (!(--n_mostres_finestra_actual)) {
                serial_envia_byte(BYTE_ESCAPAMENT);
                serial_envia_byte(0);
                n_mostres_finestra_actual = n_mostres_finestra;
                estat_trigger = e_esperant_trigger;
            }
            break;
        }
        n1_lectura = n_lectura;
    }
}