#include "serial.h"
#include <stdint.h>

void print_num_dec(uint8_t valor) {
  char s[3] = {'0', '0', '0'};
  for (uint8_t c = 2; valor > 0; c--) {
    s[c] = (valor % 10) + '0';
    valor /= 10;
  }
  serial_envia(s[0]); serial_envia(s[1]); serial_envia(s[2]);
  serial_envia('\n');
}

