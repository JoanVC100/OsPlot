export CC=avr-gcc
export CPPFLAGS=-DF_CPU=16000000UL -DSERIAL_RX_CALLBACK #-DSERIAL_CUA_TX
CFLAGS=-Wall -std=c17 -Ofast -mmcu=atmega328p -fshort-enums -flto
export CFLAGS+=--param=min-pagesize=0 # Necessari per eliminar avisos a Arch

BUILDDIR = build
LIBDIR = "../../Fitxers comuns/ArduinoUno/"
CFLAGS+= -I$(LIBDIR)
LDFLAGS=$(CFLAGS) -L$(LIBDIR)

SOURCES = $(patsubst %.c,$(BUILDDIR)/%,$(wildcard *.c))

tots: $(SOURCES)

$(BUILDDIR)/%: %.c OsPlot-MCU.h -losplot
	@mkdir -p $(BUILDDIR)
	$(CC) $? -o $@ $(CPPFLAGS) $(CFLAGS) $(LDFLAGS) -L$(LIBDIR)

$(BUILDDIR)/%: %.c -losplot
	@mkdir -p $(BUILDDIR)
	$(CC) $? -o $@ $(CPPFLAGS) $(CFLAGS) $(LDFLAGS) -L$(LIBDIR)

-losplot:
	$(MAKE) -C $(LIBDIR) libosplot.a

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
	$(MAKE) -C $(LIBDIR) neteja
	\rm -rf build
