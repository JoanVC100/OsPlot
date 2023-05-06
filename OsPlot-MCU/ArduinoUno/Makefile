CC=avr-gcc
CPPFLAGS=-DF_CPU=16000000UL -DSERIAL_RX_CALLBACK
CFLAGS=-Wall -std=c17 -Ofast -mmcu=atmega328p -fshort-enums -flto
CFLAGS+=--param=min-pagesize=0 # Necessari per eliminar avisos a Arch
LDFLAGS=$(CFLAGS)

tots: build/OsPlot-MCU build/lectura_adc
build:
	mkdir build

build/%: %.c
	$(CC) $? -o $@ $(CPPFLAGS) $(CDFLAGS) $(LDFLAGS)
build/OsPlot-MCU: OsPlot-MCU.c OsPlot-MCU.h build/serial.o build/cua.o build/adc.o build/print_num.o
build/lectura_adc: lectura_adc.c build/serial.o build/cua.o build/adc.o build/print_num.o

build/%.o: %.c
	make build
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
	\rm -rf build
