#include "serial.h"
#include <avr/io.h>
#include <avr/interrupt.h>
#include <util/atomic.h>
#define BAUD 1000000
#define USE_2X 0
#include <util/setbaud.h>

#include "cua.h"
static cua_t cua_rx, cua_tx;

void serial_obre(void) {
  cua_buida(&cua_rx);
  cua_buida(&cua_tx);

  UCSR0A = USE_2X << U2X0;
  UCSR0B = (1 << RXEN0) | (1 << TXEN0) | (1 << RXCIE0);
  UCSR0C = (1 << UCSZ01) | (1 << UCSZ00);
  UBRR0 = UBRR_VALUE;
}

void serial_tanca(void) {
  while (!cua_es_buida(&cua_tx));
  loop_until_bit_is_set(UCSR0A, UDRE0);
  loop_until_bit_is_set(UCSR0A, TXC0);
  UCSR0B &= ~((1 << RXEN0) | (1 << TXEN0) | (1 << RXCIE0));
}

ISR(USART_RX_vect, ISR_BLOCK) {
  if (!cua_es_plena(&cua_rx))
    cua_posa(&cua_rx, UDR0);
}

ISR(USART_UDRE_vect, ISR_BLOCK) {
  if ((!cua_es_buida(&cua_tx)))
    UDR0 = cua_treu(&cua_tx);
  else
    UCSR0B &= ~(1  << UDRIE0);
}

uint8_t serial_llegir(void) {
  while (cua_es_buida(&cua_rx));
  return cua_treu(&cua_rx);
}

void serial_envia_byte(uint8_t b) {
  while(cua_es_plena(&cua_tx));
  cua_posa(&cua_tx, b);
  UCSR0B |= (1  << UDRIE0);
}

void serial_envia_4byte(uint32_t b) {
  while(cua_es_plena(&cua_tx));
  for (uint8_t c = 0; c <= 3; c++) {
    cua_posa(&cua_tx, ((uint8_t*) &b)[c]);
  }
  UCSR0B |= (1  << UDRIE0);
}

bool serial_pot_llegir(void) {
  return !cua_es_buida(&cua_rx);
}

bool serial_pot_enviar(void) {
  return !cua_es_plena(&cua_tx);
}
