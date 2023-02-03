#ifndef _CUA_H_
#define _CUA_H_

#include <stdint.h>
#include <stdbool.h>

#define LONGITUD_CUA 16
typedef struct {
  uint8_t cua[LONGITUD_CUA];
  volatile uint8_t index_lectura, n_escriptures;
} cua_t;
/*
  Estructura de la cua. Està formada per una taula d'enters de 8 bits de 
  longitud LONGITUD_CUA i dos índexs; el de lectura i el nombre 
  d'escriptures fet.
*/

void cua_buida(cua_t *const q);
// Inicialitza i buida la cua.

bool cua_es_buida(const cua_t *const q);
// Retorna cert si la cua està buida.

bool cua_es_plena(const cua_t *const q);
// Retorna cert si la cua està plena.

void cua_posa(cua_t *const q, uint8_t v);
// Posa l'element 'v' a la cua.

uint8_t cua_treu(cua_t *const q);
// Treu un element de la cua.

#endif
