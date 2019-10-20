
import * as WASM from "./wasm"

onmessage = function(eventData) {

    console.log("message received in worker");

    const outputData = WASM.render(eventData.data);

    this.postMessage(outputData);
}
