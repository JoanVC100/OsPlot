#include <stdint.h>
#include <avr/interrupt.h>
#include <util/delay.h>
#include "serial.h"
#include "adc.h"
#include "print_num.h"

#include "OsPlot-MCU.h"

typedef enum {e_esperant_ordres, e_maquina_trigger} estats_osplot_mcu_t;
volatile estats_osplot_mcu_t estat_osplot_mcu = e_esperant_ordres;

#define DEFECTE_NIVELL_TRIGGER 128
#define PRESCALER_ADC p16
typedef enum {e_esperant_trigger, e_capturant} estats_trigger_t;
volatile estats_trigger_t estat_trigger = e_esperant_trigger;
volatile uint8_t nivell_trigger = DEFECTE_NIVELL_TRIGGER;
volatile uint16_t n_mostres_finestra = 500;
volatile uint8_t factor_oversampling = 1;
const float fs = F_CPU/PRESCALER_ADC/13.;

void inline maquina_osplot_mcu(void) {
    msg_capçalera_pc_t capçalera_pc = serial_llegir_byte();
    msg_tramesa_t tramesa;
    switch (capçalera_pc) {
    case PC_INICIA_TRIGGER:
        estat_osplot_mcu = e_maquina_trigger;
        break;
    case PC_CANVIAR_FACTOR_OVERSAMPLING:
        tramesa.factor_oversampling = serial_llegir_byte();
        if (tramesa.factor_oversampling <= MAXIM_OVERSAMPLING) {
            factor_oversampling = tramesa.factor_oversampling;
            serial_envia_escapament(MCU_FACTOR_OVERSAMPLING_CANVIAT);
        }
        else serial_envia_escapament(MCU_ERROR);
        break;
    case PC_RETORNA_FACTOR_OVERSAMPLING:
        serial_envia_byte(factor_oversampling);
        serial_envia_escapament(MCU_FACTOR_OVERSAMPLING);
        break;
    case PC_RETORNA_FS:
        serial_envia_4byte((uint8_t*) &fs);
        serial_envia_escapament(MCU_FS);
        break;
    case PC_CANVIAR_N_MOSTRES:
        tramesa.n_mostres = serial_llegir_2byte();
        if (tramesa.n_mostres <= MAXIM_N_MOSTRES) {
            n_mostres_finestra = tramesa.n_mostres;
            serial_envia_escapament(MCU_N_MOSTRES_CANVIADES);
        }
        else serial_envia_escapament(MCU_ERROR);
        break;
    case PC_RETORNA_N_MOSTRES:
        serial_envia_2byte((uint8_t*) &n_mostres_finestra);
        serial_envia_escapament(MCU_N_MOSTRES);
        break;
    case PC_CANVIAR_NIVELL_TRIGGER:
        nivell_trigger = serial_llegir_byte();
        serial_envia_escapament(MCU_NIVELL_TRIGGER_CANVIAT);
        break;
    case PC_RETORNA_NIVELL_TRIGGER:
        serial_envia_byte(nivell_trigger);
        serial_envia_escapament(MCU_NIVELL_TRIGGER);
        break;
    }
}

void inline maquina_trigger(void) {
    adc_inici_lectura();
    uint8_t n_lectura, n1_lectura = adc_llegeix8();
    uint16_t n_mostres_finestra_actual = n_mostres_finestra;
    adc_inici_lectura();
    while (estat_osplot_mcu == e_maquina_trigger) {
        n_lectura = adc_llegeix8();
        adc_inici_lectura();
        switch (estat_trigger) {
        case e_esperant_trigger:
            if (n1_lectura <= nivell_trigger && n_lectura >= nivell_trigger) {
                serial_envia_byte(n_lectura);
                n_mostres_finestra_actual = n_mostres_finestra-1;
                estat_trigger = e_capturant;
            }                    
            break;
        case e_capturant:
            serial_envia_byte(n_lectura);
            if (!(--n_mostres_finestra_actual)) {
                serial_envia_escapament(MCU_FINESTRA);
                n_mostres_finestra_actual = n_mostres_finestra;
                estat_trigger = e_esperant_trigger;
            }
            break;
        }
        n1_lectura = n_lectura;
    }
    serial_envia_escapament(MCU_FINESTRA);
    estat_trigger = e_esperant_trigger;
}

void inici_d_ordres(void) {
    switch (estat_osplot_mcu) {
    case e_maquina_trigger:
        estat_osplot_mcu = e_esperant_ordres;
        break;
    default: break;
    }
}

int main() {
    adc_inicia(a5, v5, PRESCALER_ADC);
    serial_rx_callback(inici_d_ordres);
    serial_obre();
    sei();

    while (1) {
        switch (estat_osplot_mcu) {
        case e_esperant_ordres:
            maquina_osplot_mcu();
            break;
        case e_maquina_trigger:
            if (factor_oversampling < 2) maquina_trigger();
            //else maquina_trigger_oversampling();
            break;
        }
    }
}