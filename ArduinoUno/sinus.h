#ifndef _SINUS_H_
#define _SINUS_H_

#include <stdint.h>

#define uTS 20

#define SINUS_FS 1000000/uTS
// Mostres generades a partir d'una Fs de 50kHz
// Sinus de 100 Hz
const uint8_t hz_100[] = {128,130,131,133,134,136,138,139,141,142,144,146,147,149,150,152,154,155,157,158,160,161,163,164,166,168,169,171,172,174,175,177,178,180,181,182,184,185,187,188,190,191,192,194,195,197,198,199,201,202,203,205,206,207,208,210,211,212,213,214,216,217,218,219,220,221,222,223,225,226,227,228,229,230,231,232,232,233,234,235,236,237,238,239,239,240,241,242,242,243,244,244,245,246,246,247,248,248,249,249,250,250,251,251,252,252,252,253,253,253,254,254,254,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,254,254,254,253,253,253,252,252,252,251,251,250,250,249,249,248,248,247,246,246,245,244,244,243,242,242,241,240,239,239,238,237,236,235,234,233,232,232,231,230,229,228,227,226,225,223,222,221,220,219,218,217,216,214,213,212,211,210,208,207,206,205,203,202,201,199,198,197,195,194,192,191,190,188,187,185,184,182,181,180,178,177,175,174,172,171,169,168,166,164,163,161,160,158,157,155,154,152,150,149,147,146,144,142,141,139,138,136,134,133,131,130,128,126,125,123,122,120,118,117,115,114,112,110,109,107,106,104,102,101,99,98,96,95,93,92,90,88,87,85,84,82,81,79,78,76,75,74,72,71,69,68,66,65,64,62,61,59,58,57,55,54,53,51,50,49,48,46,45,44,43,42,40,39,38,37,36,35,34,33,31,30,29,28,27,26,25,24,24,23,22,21,20,19,18,17,17,16,15,14,14,13,12,12,11,10,10,9,8,8,7,7,6,6,5,5,4,4,4,3,3,3,2,2,2,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,1,2,2,2,3,3,3,4,4,4,5,5,6,6,7,7,8,8,9,10,10,11,12,12,13,14,14,15,16,17,17,18,19,20,21,22,23,24,24,25,26,27,28,29,30,31,33,34,35,36,37,38,39,40,42,43,44,45,46,48,49,50,51,53,54,55,57,58,59,61,62,64,65,66,68,69,71,72,74,75,76,78,79,81,82,84,85,87,88,90,92,93,95,96,98,99,101,102,104,106,107,109,110,112,114,115,117,118,120,122,123,125,126};
// Sinus de 127 Hz
const uint8_t hz_127[] = {218,219,220,222,223,225,226,227,228,230,231,232,233,234,235,236,238,239,240,240,241,242,243,244,245,246,246,247,248,248,249,250,250,251,251,252,252,253,253,253,254,254,254,254,255,255,255,255,255,255,255,255,255,255,255,254,254,254,254,253,253,253,252,252,251,251,250,250,249,249,248,247,247,246,245,244,244,243,242,241,240,239,238,237,236,235,234,233,231,230,229,228,226,225,224,222,221,220,218,217,215,214,212,211,209,208,206,205,203,201,200,198,196,194,193,191,189,187,186,184,182,180,178,176,174,173,171,169,167,165,163,161,159,157,155,153,151,149,147,145,143,141,139,137,135,133,131,129,127,125,123,121,119,117,115,113,111,109,107,105,103,101,99,97,95,93,91,89,87,85,83,81,79,77,75,74,72,70,68,66,65,63,61,59,58,56,54,53,51,49,48,46,45,43,42,40,39,37,36,34,33,32,30,29,28,26,25,24,23,22,21,19,18,17,16,15,14,13,13,12,11,10,9,9,8,7,6,6,5,5,4,4,3,3,2,2,2,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,1,1,1,1,2,2,2,3,3,4,4,5,5,6,6,7,8,8,9,10,11,12,12,13,14,15,16,17,18,19,20,22,23,24,25,26,28,29,30,31,33,34,36,37,38,40,41,43,44,46,48,49,51,52,54,56,57,59,61,63,64,66,68,70,72,73,75,77,79,81,83,85,87,88,90,92,94,96,98,100,102,104,106,108,110,112,114,116,118,120,122,124,127,129,131,133,135,137,139,141,143,145,147,149,151,153,155,157,159,161,163,165,167,168,170,172,174,176,178,180,182,184,185,187,189,191,192,194,196,198,199,201,203,204,206,208,209,211,212,214,215,217,218};

#endif