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

#define N_MOSTRES 100
#define NIVELL_TRIGGER 128
#define TOTAL_MITJANES_F 100
#define TOTAL_MITJANES_TEMPS_BUCLE 100

void system_printf(char* format);

#include <signal.h>
static FILE* port = NULL;
static const char* s_port;
void ctrl_c_handler(int dummy) {
    if (port) {
        fclose(port);
    }
    system_printf("stty -F /dev/ttyACM0 sane > /dev/null");
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
        
        port = fopen(s_port, "r");
        char* first_s;
        size_t first_s_len = 0;
        getline(&first_s, &first_s_len, port);
        double fs = strtol(first_s, NULL, 10);
        free(first_s);
        printf("Fs: %ld\n", (unsigned long) round(fs));

        int n1_lectura = 0, total_mostres = 0;
        double mitjana_F = 0, mitjana_F_temporal = 0;
        int n_mitjanes_F = TOTAL_MITJANES_F;
#ifdef DEBUG
        double mitjana_temps_bucle = 0;
        int n_mitjanes_temps_bucle = TOTAL_MITJANES_TEMPS_BUCLE;
        struct timespec inici, final;
#endif
        while (1) {
#ifdef DEBUG
            clock_gettime(CLOCK_MONOTONIC, &inici);
#endif
            uint8_t s[N_MOSTRES*4];
            fread(s, sizeof(uint8_t), sizeof(s)/sizeof(uint8_t), port);
            for (unsigned int c = 0; c < sizeof(s)/sizeof(char); c += 4) {
                unsigned int n_lectura;
                if (sscanf((char*) s+c, "%d\n", &n_lectura) != EOF) {
                    total_mostres++;
                    if (n1_lectura <= NIVELL_TRIGGER && n_lectura >= NIVELL_TRIGGER) {
                        mitjana_F_temporal += (fs/total_mostres)/TOTAL_MITJANES_F;
                        if (!(--n_mitjanes_F)) {
                            mitjana_F = mitjana_F_temporal;
                            mitjana_F_temporal = 0;
                            n_mitjanes_F = TOTAL_MITJANES_F;
                        }
                        total_mostres = 0;
                    }
                    n1_lectura = n_lectura;
                }
            }
#ifdef DEBUG
            clock_gettime(CLOCK_MONOTONIC, &final);
            mitjana_temps_bucle += ((final.tv_sec - inici.tv_sec) * 1000000000LL + final.tv_nsec - inici.tv_nsec) / 1000000. / TOTAL_MITJANES_F;
            n_mitjanes_temps_bucle--;
            if (!n_mitjanes_temps_bucle) {
                printf("F: % 6.3f Temps de bucle: % 5.3f ms\r", mitjana_F, mitjana_temps_bucle);
                mitjana_temps_bucle = 0;
                n_mitjanes_temps_bucle = TOTAL_MITJANES_TEMPS_BUCLE;
            }
#else
            printf("F: % 6.3f\r", mitjana_F);
#endif
        }
    }
    
}