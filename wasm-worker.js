import init, {greet} from "./pkg/hello_wasm.js";
import {xxx} from "./pkg/hello_wasm.js";

await init();
await xxx(navigator.hardwareConcurrency);

var startTime = performance.now()
var proof = greet("ss");
var endTime = performance.now()

console.log(`Call to doSomething took ${endTime - startTime} milliseconds`)
console.log(proof);
