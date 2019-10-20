
import * as WASM from "./wasm"

onmessage = function(eventData) {

    WASM.render(eventData);

    console.log("message received in worker")
}
