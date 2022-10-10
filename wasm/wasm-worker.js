import init, {init_threads, contribute, hash_entropy} from "./pkg/kate_ptau_rs.js";

console.log("available threads:", navigator.hardwareConcurrency);

await init();
await init_threads(navigator.hardwareConcurrency);

fetch('./initialContribution.json').then(response => {
    response.json().then(data => {
        var json_string = JSON.stringify(data);
        var startTime = performance.now()
        console.log("start");
        try {
            var entropy = hash_entropy("aaaaa");
            var res = contribute(entropy, json_string);
        } catch (e) {
            // handle error
        }
        var endTime = performance.now()
        console.log(`Contribution took ${endTime - startTime} milliseconds`);
    });
});