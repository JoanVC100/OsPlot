CC=avr-gcc
CPPFLAGS=-DF_CPU=16000000UL
CFLAGS=-Wall -std=c17 -Ofast -mmcu=atmega328p -fshort-enums -flto
CFLAGS+=--param=min-pagesize=0 # Necessari per eliminar avisos a Arch
LDFLAGS=$(CFLAGS)

tots: lectura_adc serra sinus sinus_artefacte pols pols_generat

lectura_adc: lectura_adc.c serial.o cua.o adc.o print_num.o
serra: serra.c serial.o cua.o print_num.o
sinus: sinus.c serial.o cua.o print_num.o
sinus_artefacte: sinus_artefacte.c serial.o cua.o print_num.o
pols: pols.c serial.o cua.o print_num.o
pols_generat: pols_generat.c serial.o cua.o adc.o print_num.o

serial.o: serial.c serial.h cua.h
cua.o: cua.c cua.h
adc.o: adc.c adc.h
print_num.o: print_num.c print_num.h serial.h

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
	\rm -f *~ *.o lectura_adc serra sinus sinus_artefacte pols pols_generat
