#ifndef _PRINT_NUM_H_
#define _PRINT_NUM_H_

#include <stdint.h>

void print_num_dec(uint8_t valor);
/* Envia per serial un número en decimal.
   El màxim admés per aquesta funció són
   tres xifres (0-999). */

void print_num_dec6(uint32_t valor);
/* Envia per serial un número en decimal.
   El màxim admés per aquesta funció són
   tres xifres (0-999999). */

#endif
