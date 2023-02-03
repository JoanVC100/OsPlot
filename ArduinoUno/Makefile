CC=avr-gcc
CPPFLAGS=-DF_CPU=16000000UL
CFLAGS=-Wall -std=c17 -Ofast -mmcu=atmega328p -fshort-enums
CFLAGS+=--param=min-pagesize=0 # Necessari per eliminar avisos a Arch
LDFLAGS=$(CFLAGS)

main: main.c serial.o cua.o

serial.o: serial.c serial.h cua.h
cua.o: cua.c cua.h

.PHONY: grava_arduino neteja

PORT ?= /dev/ttyACM0
BAUD ?= 115200
grava_arduino: main
	avrdude -c arduino -p atmega328p -P $(PORT) -U main -b $(BAUD)

neteja:
	\rm -f *~ *.o main
