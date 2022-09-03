import init, {init_threads, contribute_wasm} from "./pkg/kate_ptau_rs.js";

console.log("available threads:", navigator.hardwareConcurrency);

await init();
await init_threads(navigator.hardwareConcurrency);

fetch('./initialContribution.json').then(response => {
    response.json().then(data => {
        var json_string = JSON.stringify(data);
        var startTime = performance.now()
        console.log("start");
        var res = contribute_wasm(json_string);
        var endTime = performance.now()
        console.log(`Contribution took ${endTime - startTime} milliseconds`)
    });
});