#include <stddef.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <stdint.h>
#include <unistd.h>
#include <string.h>
#include <math.h>

//#define DEBUG

#define BYTE_ESCAPAMENT 128
#define N_MAXIM_MOSTRES 1000
#define NOM_CUA "osplot.pipe"

void system_printf(char* format);

#include <signal.h>
FILE* cua = NULL;
FILE* port = NULL;
const char* s_port;
void ctrl_c_handler(__attribute__ ((unused) )int dummy) {
    if (cua) {
        fclose(cua);
    }
    unlink(NOM_CUA);
    if (port) {
        fclose(port);
    }
    system_printf("stty -F /dev/ttyACM0 sane > /dev/null");
    exit(0);
}

void system_printf(char* format) {
    char s[4096];
    snprintf(s, sizeof(s)/sizeof(char), format, s_port);
    system(s);
}

int main(int argc, char* argv[]) {
    signal(SIGINT, ctrl_c_handler);

    if (argc < 2) {
        printf("No s'ha donat el port.\n");
    }
    else if (argc > 2) {
        printf("S'han donat arguments extra.\n");
    }
    else {
        s_port = argv[1];
        if (access(s_port, F_OK) != 0) {
            printf("El port %s no existeix\n", s_port);
            exit(1);
        }
        system_printf("stty -F %s sane > /dev/null");
        system_printf("stty -F %s raw speed 1000000 -echo > /dev/null");
        mkfifo(NOM_CUA, 0600);
#ifndef DEBUG
        system("gnuplot -e 'fs=9615' plot.gnu &");
#else
        system("gnuplot plot_debug.gnu &");
#endif
        port = fopen(s_port, "rb");

        uint8_t dades[N_MAXIM_MOSTRES];
        while (1) {
            int c, cnext;
            size_t i = 0;
            do {
                c = getc(port);
                if (c == BYTE_ESCAPAMENT) {
                    cnext = getc(port);
                    if (cnext != BYTE_ESCAPAMENT) break;
                }
                dades[i++] = c;
            }
            while (c != EOF && i < N_MAXIM_MOSTRES);
            cua = fopen(NOM_CUA, "wb");
            fwrite(dades, sizeof(uint8_t), i, cua);
            fclose(cua);
            cua = NULL;
#ifdef DEBUG
            printf("%ld\n-\n", i);
#endif
        }
    }
    
}