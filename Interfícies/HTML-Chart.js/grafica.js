const ctx = document.getElementById("grafica").getContext("2d");
const grafica = new Chart(ctx, {
    type: "line",
    data: {
        labels: [...Array(500).keys()],
        datasets: [{
            label: "OsPlot",
            borderColor: "rgb(75, 192, 192)",
        }]
    },
    options: {
        scales: {
            y: {
                beginAtZero: true,
                max: 255
            }
        }
    }
});
grafica.options.animation = false; // disables all animations
grafica.options.animations.colors = false; // disables animation defined by the collection of 'colors' properties
grafica.options.animations.x = false; // disables animation defined by the 'x' property
grafica.options.transitions.active.animation.duration = 0; // disables the animation for 'active' mode

let socket = new WebSocket("ws://localhost:26142");
socket.binaryType = "arraybuffer"
socket.onopen = function(event) {
    console.log("Connexió establerta");
};

let OsPlotMsgServidor;
protobuf.load("missatges.proto", function(err, root) {
    if (err)
        throw err;
    else
        OsPlotMsgServidor = root.lookupType("OsPlot.OsPlotMsgServidor");
})

socket.onmessage = function(event) {
    const start = performance.now();

    let msg = OsPlotMsgServidor.decode(new Uint8Array(event.data));

    switch (msg.tipus) {
    case OsPlotMsgServidor.Tipus_Msg.Mostres:
        grafica.data.datasets[0].data = msg.mostres;
        grafica.update();
        break;
    case OsPlotMsgServidor.Tipus_Msg.Connexio_Inicial_OK:
        break;
    default:
        console.log("Tipus de missatge desconegut: "+$msg.tipus);
    }

    const end = performance.now();
    console.log("Temps d'execució: "+(end-start)+" ms");
};

socket.onclose = function(event) {
    if (event.wasClean) {
        console.log("S'ha tancat la connexió");
    }
    else {
        console.log("S'ha caigut la connexió");
    }
};

socket.onerror = function(error) {
    console.log(error);
};