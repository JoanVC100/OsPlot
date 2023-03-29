import asyncio
import websockets
import aioserial
from missatges_pb2 import *

directori_treball="/tmp/OsPlot"
defecte_n_mostres=500 # 500 mostres per defecte
defecte_nivell_trigger=128 # 128 per defecte
debug=False # True per habilitar, False per deshabilitar

cua_dades="{}/osplot.dat".format(directori_treball)
conf_n_mostres="{}/mostres_finestra".format(directori_treball)
conf_nivell_trigger="{}/nivell_trigger".format(directori_treball)
connectats = set()

def inicia_trigger(event_loop):
    try:
        port_serial = aioserial.AioSerial(port="/dev/ttyACM0", baudrate=1000000, timeout=3)
        if (debug): event_loop.create_task(trigger_debug(port_serial))
        else: event_loop.create_task(trigger(port_serial))
    except Exception as e:
        print("No s'ha pogut iniciar la connexió Serial:\n{}" \
        .format(e))
        return False
    return True

from enum import Enum
Trigger = Enum("Trigger", ["ESPERANT_TRIGGER", "CAPTURANT"])
async def trigger(port_serial: aioserial.AioSerial):
    n_mostres_a_llegir = 0
    nivell_trigger = defecte_nivell_trigger
    n1_lectura = 0
    lectures = bytearray(defecte_n_mostres)
    msg = OsPlotMsgServidor()
    msg.tipus = OsPlotMsgServidor.Mostres

    await asyncio.sleep(3) # Espera a que l'Arduino es reinici
    await port_serial.read_async(100000)
    await port_serial.write_async(b'S')
    dades: bytes = await port_serial.read_async(4) # La primera lectura és la Fs
    fs = int.from_bytes(dades, "little")

    estat = Trigger.ESPERANT_TRIGGER
    while True:
        dades: bytes = await port_serial.read_async(100)
        for n_lectura in dades:

            if estat == Trigger.ESPERANT_TRIGGER:
                if n1_lectura <= nivell_trigger and n_lectura >= nivell_trigger:
                    estat=Trigger.CAPTURANT
                n1_lectura = n_lectura

            elif estat == Trigger.CAPTURANT:
                lectures[n_mostres_a_llegir] = n_lectura
                n_mostres_a_llegir += 1
                if n_mostres_a_llegir == defecte_n_mostres:
                    msg.mostres = bytes(lectures)
                    websockets.broadcast(connectats, msg.SerializeToString())
                    n_mostres_a_llegir = 0
                    estat=Trigger.ESPERANT_TRIGGER
                n1_lectura=n_lectura

async def trigger_debug(port_serial: aioserial.AioSerial):
    import time
    n_mostres_a_llegir = 0
    nivell_trigger = defecte_nivell_trigger
    n1_lectura = 0
    lectures = bytearray(defecte_n_mostres)
    msg = OsPlotMsgServidor()
    msg.tipus = OsPlotMsgServidor.Mostres

    await asyncio.sleep(3) # Espera a que l'Arduino es reinici
    await port_serial.read_async(100000)
    await port_serial.write_async(b'S')
    dades: bytes = await port_serial.read_async(4) # La primera lectura és la Fs
    fs = int.from_bytes(dades, "little")

    estat = Trigger.ESPERANT_TRIGGER
    temps_inici = None
    while True:
        dades: bytes = await port_serial.read_async(100)
        for n_lectura in dades:

            if estat == Trigger.ESPERANT_TRIGGER:
                if n1_lectura <= nivell_trigger and n_lectura >= nivell_trigger:
                    estat=Trigger.CAPTURANT
                n1_lectura = n_lectura
                temps_inici = time.time()

            elif estat == Trigger.CAPTURANT:
                lectures[n_mostres_a_llegir] = n_lectura
                n_mostres_a_llegir += 1
                if n_mostres_a_llegir == defecte_n_mostres:
                    msg.mostres = bytes(lectures)
                    websockets.broadcast(connectats, msg.SerializeToString())
                    print((time.time()-temps_inici)*1000)
                    n_mostres_a_llegir = 0
                    estat=Trigger.ESPERANT_TRIGGER
                n1_lectura=n_lectura

async def handler(websocket):
    connectats.add(websocket)
    while True:
        try:
            message = await websocket.recv()
        except websockets.ConnectionClosedOK:
            connectats.remove(websocket)
            break
        print(message)

async def main():
    trigger_iniciat = inicia_trigger(asyncio.get_event_loop())
    if (trigger_iniciat):
        async with websockets.serve(handler, "localhost", 26142):
            await asyncio.Future()  # run forever
asyncio.run(main())