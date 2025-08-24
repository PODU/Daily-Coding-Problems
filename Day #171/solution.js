// Busiest period: sort events by timestamp, sweep occupancy, track max-occupancy interval. O(n log n) time, O(n) space.

const ENTER = "enter", EXIT = "exit";

function busiestPeriod(events) {
    const sorted = events.slice().sort((a, b) => a[0] - b[0]);
    let occ = 0, maxOcc = -1, best = [0, 0];
    for (let i = 0; i < sorted.length; i++) {
        const [ts, count, type] = sorted[i];
        occ += (type === ENTER) ? count : -count;
        if (occ > maxOcc && i + 1 < sorted.length) {
            maxOcc = occ;
            best = [ts, sorted[i + 1][0]];
        }
    }
    return best;
}

function main() {
    const events = [
        [1526579928, 3, ENTER],
        [1526580382, 2, EXIT],
        [1526579999, 1, ENTER],
        [1526580001, 5, ENTER],
    ];
    const [start, end] = busiestPeriod(events);
    console.log(`(${start}, ${end})`);
}

main();
