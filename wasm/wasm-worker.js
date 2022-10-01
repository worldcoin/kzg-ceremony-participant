import init, {init_threads, contribute, get_entropy} from "./pkg/kate_ptau_rs.js";

console.log("available threads:", navigator.hardwareConcurrency);

await init();
await init_threads(navigator.hardwareConcurrency);

fetch('./initialContribution.json').then(response => {
    response.json().then(data => {
        var json_string = JSON.stringify(data);
        var startTime = performance.now()
        console.log("start");
        var entropy = get_entropy("aaaaa");
        console.log("entropy", entropy);
        var res = contribute(entropy, json_string);
        var endTime = performance.now()
        console.log(`Contribution took ${endTime - startTime} milliseconds`)
    });
});