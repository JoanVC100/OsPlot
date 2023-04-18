#ifndef _OSPLOT_MCU_H_
#define _OSPLOT_MCU_H_

#include <stdint.h>

typedef enum {
    C_PC_INICIA_TRIGGER = 0,
    C_PC_CANVIAR_FS,
    C_PC_RETORNA_POSSIBLES_FS,
    C_PC_CANVIAR_N_MOSTRES,
    C_PC_RETORNA_N_MOSTRES
} msg_capçalera_pc_t;

typedef enum {
    C_MCU_OK = 129,
    C_MCU_ERROR,
    C_MCU_RETORNA_FSS,
    C_MCU_RETORNA_N_MOSTRES
} msg_capçalera_mcu_t;

typedef union {
    uint32_t fss[16];
    uint16_t n_mostres;
    uint8_t index_fs;
} msg_tramesa_t;

#define FINAL_FSS 0

#endif