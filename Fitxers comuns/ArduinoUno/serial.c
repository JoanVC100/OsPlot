#include "serial.h"
#include <avr/io.h>
#include <avr/interrupt.h>
#include <util/atomic.h>
#define BAUD 1000000
#define USE_2X 0
#include <util/setbaud.h>

#include "cua.h"
static cua_t cua_rx;
#ifdef SERIAL_CUA_TX
static cua_t cua_tx;
#endif

#ifndef BYTE_ESCAPAMENT
#error "No s'ha definit byte d'escapament"
#endif

void serial_obre(void) {
  cua_buida(&cua_rx);
#ifdef SERIAL_CUA_TX
  cua_buida(&cua_tx);
#endif
  UCSR0A = USE_2X << U2X0;
  UCSR0B = (1 << RXEN0) | (1 << TXEN0) | (1 << RXCIE0);
  UCSR0C = (1 << UCSZ01) | (1 << UCSZ00);
  UBRR0 = UBRR_VALUE;
}

void serial_tanca(void) {
#ifdef SERIAL_CUA_TX
  while (!cua_es_buida(&cua_tx));
#endif
  loop_until_bit_is_set(UCSR0A, UDRE0);
  loop_until_bit_is_set(UCSR0A, TXC0);
  UCSR0B &= ~((1 << RXEN0) | (1 << TXEN0) | (1 << RXCIE0));
}

#ifdef SERIAL_RX_CALLBACK
#include <stddef.h>
volatile serial_rx_cb_t rx_cb = NULL;
void serial_rx_callback(serial_rx_cb_t cb) {
  rx_cb = cb;
}
#endif
ISR(USART_RX_vect, ISR_BLOCK) {
  if (!cua_es_plena(&cua_rx))
    cua_posa(&cua_rx, UDR0);
#ifdef SERIAL_RX_CALLBACK
  if (rx_cb) rx_cb();
#endif
}

#ifdef SERIAL_CUA_TX
ISR(USART_UDRE_vect, ISR_BLOCK) {
  if ((!cua_es_buida(&cua_tx)))
    UDR0 = cua_treu(&cua_tx);
  else
    UCSR0B &= ~(1  << UDRIE0);
}
#endif

uint8_t serial_llegir_byte(void) {
  while (cua_es_buida(&cua_rx));
  return cua_treu(&cua_rx);
}

uint16_t serial_llegir_2byte(void) {
  while (elements_cua(&cua_rx) < 1);
  return cua_treu(&cua_rx) + (cua_treu(&cua_rx) << 8);
}

void serial_envia_byte(uint8_t b) {
#ifdef SERIAL_CUA_TX
  while(cua_es_plena(&cua_tx));
  cua_posa(&cua_tx, b);
  UCSR0B |= (1  << UDRIE0);
  if (b == BYTE_ESCAPAMENT) {
    while(cua_es_plena(&cua_tx));
    cua_posa(&cua_tx, b);
  }
#else
  loop_until_bit_is_set(UCSR0A, UDRE0);
  UDR0 = b;
  if (b == BYTE_ESCAPAMENT) {
    loop_until_bit_is_set(UCSR0A, UDRE0);
    UDR0 = b;
  }
#endif
}

void serial_envia_escapament(uint8_t b) {
#ifdef SERIAL_CUA_TX
  while(cua_es_plena(&cua_tx));
  cua_posa(&cua_tx, BYTE_ESCAPAMENT);
  while(cua_es_plena(&cua_tx));
  cua_posa(&cua_tx, b);
  UCSR0B |= (1  << UDRIE0);
#else
  loop_until_bit_is_set(UCSR0A, UDRE0);
  UDR0 = BYTE_ESCAPAMENT;
  loop_until_bit_is_set(UCSR0A, UDRE0);
  UDR0 = b;
#endif
}

void serial_envia_2byte(uint8_t* b) {
#ifdef SERIAL_CUA_TX
  for (uint8_t c = 0; c <= 1; c++) {
    while(cua_es_plena(&cua_tx));
    cua_posa(&cua_tx, b[c]);
    if (b[c] == BYTE_ESCAPAMENT) {
      while(cua_es_plena(&cua_tx));
      cua_posa(&cua_tx, b[c]);
    }
  }
  UCSR0B |= (1  << UDRIE0);
#else
  for (uint8_t c = 0; c <= 1; c++) {
    loop_until_bit_is_set(UCSR0A, UDRE0);
    UDR0 = b[c];
    if (b[c] == BYTE_ESCAPAMENT) {
      loop_until_bit_is_set(UCSR0A, UDRE0);
    UDR0 = b[c];
    }
  }
#endif
}

void serial_envia_4byte(uint8_t* b) {
#ifdef SERIAL_CUA_TX
  for (uint8_t c = 0; c <= 3; c++) {
    while(cua_es_plena(&cua_tx));
    cua_posa(&cua_tx, b[c]);
    if (b[c] == BYTE_ESCAPAMENT) {
      while(cua_es_plena(&cua_tx));
      cua_posa(&cua_tx, b[c]);
    }
  }
  UCSR0B |= (1  << UDRIE0);
#else
  for (uint8_t c = 0; c <= 3; c++) {
    loop_until_bit_is_set(UCSR0A, UDRE0);
    UDR0 = b[c];
    if (b[c] == BYTE_ESCAPAMENT) {
      loop_until_bit_is_set(UCSR0A, UDRE0);
    UDR0 = b[c];
    }
  }
#endif
}

bool serial_pot_llegir(void) {
  return !cua_es_buida(&cua_rx);
}

bool serial_pot_enviar(void) {
#ifdef SERIAL_CUA_TX
  return !cua_es_plena(&cua_tx);
#else
  return UCSR0A && (1 << UDRE0);
#endif
}
