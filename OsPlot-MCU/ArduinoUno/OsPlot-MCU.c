#include <stdint.h>
#include <avr/interrupt.h>
#include <util/delay.h>
#include "serial.h"
#include "adc.h"
#include "print_num.h"

#include "OsPlot-MCU.h"


typedef enum {e_esperant_ordres, e_maquina_trigger} estats_osplot_mcu_t;
volatile estats_osplot_mcu_t estat_osplot_mcu = e_esperant_ordres;

#define BYTE_ESCAPAMENT 128
#define DEFECTE_NIVELL_TRIGGER 128
#define DEFECTE_PRESCALER_ADC p128
typedef enum {e_esperant_trigger, e_capturant} estats_trigger_t;
volatile estats_trigger_t estat_trigger = e_esperant_trigger;
volatile uint8_t nivell_trigger = DEFECTE_NIVELL_TRIGGER;
volatile uint16_t n_mostres_finestra = 500;
volatile adc_prescaler_t prescaler_adc = DEFECTE_PRESCALER_ADC;

#define NOMBRE_FSS 7
const adc_prescaler_t prescalers[NOMBRE_FSS] = 
{p128, p64, p32, p16, p8, p4, p2};

void inline maquina_osplot_mcu(void) {
    msg_capçalera_pc_t capçalera_pc = serial_llegir_byte();
    msg_tramesa_t tramesa;
    switch (capçalera_pc) {
    case C_PC_INICIA_TRIGGER:
        serial_envia_byte(C_MCU_OK);
        estat_osplot_mcu = e_maquina_trigger;
        break;
    case C_PC_CANVIAR_FS:
        tramesa.index_fs = serial_llegir_byte();
        if (tramesa.index_fs < NOMBRE_FSS) {
            prescaler_adc = prescalers[tramesa.index_fs];
            serial_envia_byte(C_MCU_OK);
        }
        else serial_envia_byte(C_MCU_ERROR);
        break;
    case C_PC_RETORNA_POSSIBLES_FS:
        serial_envia_byte(C_MCU_RETORNA_FSS);
        for (uint8_t c = 0; c < NOMBRE_FSS; c++) {
            serial_envia_4byte(ADC_CALCULA_FS(prescalers[c]));
        }
        serial_envia_4byte(FINAL_FSS);
        break;
    case C_PC_CANVIAR_N_MOSTRES:
        tramesa.n_mostres = serial_llegir_2byte();
        if (tramesa.n_mostres <= 1000) {
            n_mostres_finestra = tramesa.n_mostres;
            serial_envia_byte(C_MCU_OK);
        }
        else serial_envia_byte(C_MCU_ERROR);
        break;
    case C_PC_RETORNA_N_MOSTRES:
        serial_envia_byte(C_MCU_RETORNA_N_MOSTRES);
        serial_envia_2byte(n_mostres_finestra);
        break;
    }
}

void inline maquina_trigger(void) {
    adc_inicia(a5, v5, prescaler_adc);
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
                if (n_lectura == BYTE_ESCAPAMENT)
                    serial_envia_byte(BYTE_ESCAPAMENT);
                n_mostres_finestra_actual = n_mostres_finestra-1;
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
    estat_trigger = e_esperant_trigger;
    adc_atura();
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
    serial_obre();
    sei();

    while (1) {
        switch (estat_osplot_mcu) {
        case e_esperant_ordres:
            maquina_osplot_mcu();
            break;
        case e_maquina_trigger:
            maquina_trigger();
            break;
        }
    }
}