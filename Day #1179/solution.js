// Day 1179: Debounce - call f only after N ms elapse with no further invocations.
// Each call clears the pending setTimeout and schedules a new one.
// Time O(1) per call.

function debounce(f, N) {
    let timer = null;
    return function (...args) {
        if (timer) clearTimeout(timer);
        timer = setTimeout(() => {
            timer = null;
            f.apply(this, args);
        }, N);
    };
}

let count = 0;
const debounced = debounce(() => { count++; }, 100);

let i = 0;
const iv = setInterval(() => {          // rapid burst, every 20ms
    debounced(i++);
    if (i === 5) clearInterval(iv);
}, 20);

setTimeout(() => {
    console.log(`f executed ${count} time(s)`); // 1
}, 400);
