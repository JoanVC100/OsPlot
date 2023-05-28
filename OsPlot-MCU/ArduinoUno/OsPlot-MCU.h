#ifndef _OSPLOT_MCU_H_
#define _OSPLOT_MCU_H_

#include <stdint.h>

typedef enum {
    PC_INICIA_TRIGGER = 0,
    PC_CANVIAR_FACTOR_OVERSAMPLING,
    PC_RETORNA_FACTOR_OVERSAMPLING,
    PC_CANVIAR_N_MOSTRES,
    PC_RETORNA_N_MOSTRES,
    PC_RETORNA_FS,
} msg_capçalera_pc_t;

typedef enum {
    MCU_FINESTRA = 129,
    MCU_FACTOR_OVERSAMPLING_CANVIAT,
    MCU_FACTOR_OVERSAMPLING,
    MCU_N_MOSTRES_CANVIADES,
    MCU_N_MOSTRES,
    MCU_FS,
    MCU_ERROR = 255
} msg_capçalera_mcu_t;

typedef union {
    uint16_t n_mostres;
    uint8_t factor_oversampling;
} msg_tramesa_t;

#define MAXIM_N_MOSTRES 1000
#define MAXIM_OVERSAMPLING 20

#endif