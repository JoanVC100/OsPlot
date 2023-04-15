import serial
import time

port_serial = serial.Serial("/dev/ttyACM0", 1000000, timeout=3)

n_mostres = 500
nivell_trigger = 128
total_mitjanes_F = 100; total_mitjanes_temps_bucle = 100
debug = False

time.sleep(3) # Espera a que l'Arduino es reinici
port_serial.read(100000)
port_serial.write(b'S')
fs = int.from_bytes(port_serial.read(4), "little")
# La primera lectura és la Fs
print("Fs: {}".format(fs))

total_mostres = 0; n1_lectura = 0
mitjana_F = 0; mitjana_F_temporal = 0
mitjana_temps_bucle = 0
n_mitjanes_F = total_mitjanes_F; n_mitjanes_temps_bucle = total_mitjanes_temps_bucle
try:
    while(1):
        if debug: t = time.time()
        while (port_serial.in_waiting < n_mostres): pass
        dades = port_serial.read(n_mostres)
        for n_lectura in dades:
            total_mostres += 1
            if n1_lectura <= nivell_trigger and n_lectura >= nivell_trigger:
                mitjana_F_temporal += (fs/total_mostres)/total_mitjanes_F
                n_mitjanes_F -= 1
                if not n_mitjanes_F:
                    mitjana_F = mitjana_F_temporal
                    mitjana_F_temporal = 0
                    n_mitjanes_F = total_mitjanes_F
                total_mostres = 0
            n1_lectura = n_lectura

        if debug:
            mitjana_temps_bucle += ((time.time()-t)*1000) / total_mitjanes_temps_bucle
            n_mitjanes_temps_bucle -= 1
            if not n_mitjanes_temps_bucle:
                print(" "*44, end='\r')
                print("F: {:.3f}, Temps de bucle: {:.5f} ms".format(mitjana_F, mitjana_temps_bucle), end='\r')
                mitjana_temps_bucle = 0
                n_mitjanes_temps_bucle = total_mitjanes_temps_bucle
        else:
            print(" "*44, end='\r')
            print("F: {:.3f}".format(mitjana_F), end='\r')

except KeyboardInterrupt:
    print("\nFinalitzant el programa...")
    port_serial.close()