CC=avr-gcc
CPPFLAGS=-DF_CPU=16000000UL
CFLAGS=-Wall -std=c17 -Ofast -mmcu=atmega328p -fshort-enums -flto
CFLAGS+=--param=min-pagesize=0 # Necessari per eliminar avisos a Arch
LDFLAGS=$(CFLAGS)

tots: build build/lectura_adc build/serra build/sinus build/sinus_artefacte build/pols build/pols_generat
build:
	mkdir build

build/%: %.c
	$(CC) $? -o $@ $(CPPFLAGS) $(CDFLAGS) $(LDFLAGS)
build/lectura_adc: lectura_adc.c build/serial.o build/cua.o build/adc.o build/print_num.o
build/serra: serra.c build/serial.o build/cua.o build/print_num.o
build/sinus: sinus.c sinus.h build/serial.o build/cua.o build/print_num.o
build/sinus_artefacte: sinus_artefacte.c sinus.h build/serial.o build/cua.o build/print_num.o
build/pols: pols.c build/serial.o build/cua.o build/print_num.o
build/pols_generat: pols_generat.c build/serial.o build/cua.o build/adc.o build/print_num.o

build/%.o: %.c
	$(CC) -c $< -o $@ $(CPPFLAGS) $(CFLAGS)
build/serial.o: serial.c serial.h cua.h
build/cua.o: cua.c cua.h
build/adc.o: adc.c adc.h
build/print_num.o: print_num.c print_num.h serial.h

.PHONY: grava_arduino neteja
PORT ?= /dev/ttyACM0
BAUD ?= 115200
grava_arduino:
ifdef ARXIU
	make $(ARXIU)
	avrdude -c arduino -p atmega328p -P $(PORT) -U $(ARXIU) -b $(BAUD)
else
	@echo "No s'ha donat un arxiu per gravar"
endif

neteja:
	\rm -rf build/
