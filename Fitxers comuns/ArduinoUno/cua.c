#include "cua.h"
#include <stdint.h>
#include <util/atomic.h>

void cua_buida(cua_t *const q) {
  ATOMIC_BLOCK(ATOMIC_RESTORESTATE) {
    q->index_lectura = 0;
    q->n_escriptures = 0;
  }
}

uint8_t elements_cua(const cua_t *const q) {
  return q->n_escriptures;
}

bool cua_es_buida(const cua_t *const q) {
  return (q->n_escriptures == 0);
}

bool cua_es_plena(const cua_t *const q) {
  return (q->n_escriptures == LONGITUD_CUA);
}

void cua_posa(cua_t *const q, uint8_t v) {
  ATOMIC_BLOCK(ATOMIC_RESTORESTATE) {
    q->cua[(q->index_lectura + q->n_escriptures++) % LONGITUD_CUA] = v;
  }
}

uint8_t cua_treu(cua_t *const q) {
  uint8_t ret;
  ATOMIC_BLOCK(ATOMIC_RESTORESTATE) {
    ret = q->cua[q->index_lectura];
    q->index_lectura = (q->index_lectura+1) % LONGITUD_CUA;
    q->n_escriptures--;
  }
  return ret;
}