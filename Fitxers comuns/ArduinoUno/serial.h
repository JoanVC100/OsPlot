#ifndef _SERIAL_H_
#define _SERIAL_H_

#include <stdint.h>
#include <stdbool.h>

void serial_obre(void);
/* Deixa el serial a punt per enviar/rebre caràcters de 8 bits a 
   9600 bits/s, 1 bit de stop, sense paritat i en mode asíncron. */

void serial_tanca(void);
/* Deshabilita el serial per no rebre ni transmetre res. Si queden 
   bytes a la sortida, s'espera a que tot s'enviï abans de sortir. */

uint8_t serial_llegir_byte(void);
/* Retorna un byte llegit de la cua de recepció. Es bloqueja si no hi 
   ha bytes per llegir i espera fins a llegir-ne un. */

uint16_t serial_llegir_2byte(void);
/* Retorna dos byte llegits de la cua de recepció. Es bloqueja si no hi 
   ha bytes per llegir i espera fins a llegir-ne dos. */

void serial_envia_byte(uint8_t b);
/* Envia un byte pel port sèrie. En cas que la cua sigui plena, es 
   bloqueja fins que el byte es pot posar a la cua. */

#ifdef BYTE_ESCAPAMENT
void serial_envia_escapament(uint8_t b);
/* Envia un byte pel port sèrie. En cas que la cua sigui plena, es 
   bloqueja fins que el byte es pot posar a la cua. */
#endif

void serial_envia_2byte(uint8_t* b);
/* Envia dos bytes pel port sèrie. En cas que la cua sigui plena, es 
   bloqueja fins que els bytes es poden posar a la cua. */

void serial_envia_4byte(uint8_t* b);
/* Envia quatre bytes pel port sèrie. En cas que la cua sigui plena, es 
   bloqueja fins que els bytes es poden posar a la cua. */

bool serial_pot_llegir(void);
// Retorna cert si hi ha algun byte a la cua de recepció per ser llegit.

bool serial_pot_enviar(void);
// Retorna cert si és possible afegir un byte a la cua de transmissió.

#ifdef SERIAL_RX_CALLBACK
typedef void (*serial_rx_cb_t)(void);
void serial_rx_callback(serial_rx_cb_t cb);
#endif

#endif
