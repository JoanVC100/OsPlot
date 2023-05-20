LTO_PLUGIN=/usr/lib/gcc/avr/12.2.0/liblto_plugin.so
AR = avr-ar --plugin "$(LTO_PLUGIN)"

libosplot.a: libosplot.a($(patsubst %.c,%.o, $(wildcard *.c)))

neteja:
	\rm -rf libosplot.a