#include <stdint.h>
#include "serial.h"

void print_num_dec(uint8_t valor) {
  char s[3] = {'0', '0', '0'};
  for (uint8_t c = sizeof(s)-1; valor > 0; c--) {
    s[c] = (valor % 10) + '0';
    valor /= 10;
  }
  serial_envia(s[0]); serial_envia(s[1]); serial_envia(s[2]);
  serial_envia('\n');
}

void print_num_dec6(uint32_t valor) {
  char s[6] = {'0', '0', '0', '0', '0', '0'};
  for (uint8_t c = sizeof(s)-1; valor > 0; c--) {
    s[c] = (valor % 10) + '0';
    valor /= 10;
  }
  serial_envia(s[0]); serial_envia(s[1]); serial_envia(s[2]);
  serial_envia(s[3]); serial_envia(s[4]); serial_envia(s[5]);
  serial_envia('\n');
}

