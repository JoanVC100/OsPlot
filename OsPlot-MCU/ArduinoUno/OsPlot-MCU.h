#ifndef _OSPLOT_MCU_H_
#define _OSPLOT_MCU_H_

#include <stdint.h>

typedef enum {
    C_PC_INICIA_TRIGGER = 0,
    C_PC_CANVIAR_FACTOR_OVERSAMPLING,
    C_PC_RETORNA_FACTOR_OVERSAMPLING,
    C_PC_RETORNA_FS,
    C_PC_CANVIAR_N_MOSTRES,
    C_PC_RETORNA_N_MOSTRES
} msg_capçalera_pc_t;

typedef enum {
    C_MCU_OK = 129,
    C_MCU_ERROR
} msg_capçalera_mcu_t;

typedef union {
    uint16_t n_mostres;
    uint8_t factor_oversampling;
} msg_tramesa_t;

#define MAXIM_N_MOSTRES 1000
#define MAXIM_OVERSAMPLING 20

#define BYTE_ESCAPAMENT 128

#endif