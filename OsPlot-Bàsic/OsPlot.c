#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>
#include <sys/types.h>
#include <sys/stat.h>

#define N_MOSTRES 500
#define NOM_CUA "osplot.pipe"

void system_printf(char* format);

#include <signal.h>
static FILE* port = NULL;
static const char* s_port;
void ctrl_c_handler(int dummy) {
    if (port) {
        fclose(port);
    }
    system_printf("stty -F %s sane > /dev/null");
    exit(0);
}

void system_printf(char* format) {
    char s[4096];
    snprintf(s, sizeof(s), format, s_port);
    system(s);
}

int main(int argc, char* argv[]) {
    signal(SIGINT, ctrl_c_handler);
    if( argc < 2 ) {
        printf("No s'ha donat el port.\n");
    }
    else if( argc > 2 ) {
        printf("S'han donat arguments extra. .\n");
    }
    else {
        s_port = argv[1];
        system_printf("stty -F %s sane > /dev/null");
        system_printf("stty -F %s raw speed 1000000 -echo > /dev/null");

		system("gnuplot -e \"cua_lectura='" NOM_CUA "'; fs=16000000/128/13\" plot.gnu &");
        
		mkfifo(NOM_CUA, 0666);
        port = fopen(s_port, "rb");
		uint8_t s[N_MOSTRES];
        while (1) {
            fread(s, sizeof(uint8_t), sizeof(s)/sizeof(uint8_t), port);
			FILE* cua = fopen(NOM_CUA, "wb");
			fwrite(s, sizeof(uint8_t), sizeof(s)/sizeof(uint8_t), cua);
			fclose(cua);
		}
	}
}