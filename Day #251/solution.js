// External/bucket sort demo for sorting ~1M ints in [0,1e9] with bounded memory.
// Real answer for data exceeding RAM: external merge sort (split into chunks ->
// sort each chunk on disk -> k-way merge). Here we bucket by range, sort each
// bucket, and concatenate. Time: O(n log(n/k)); Space: O(n) bounded per bucket.

function bucketSort(data, maxVal, numBuckets) {
    const width = Math.floor(maxVal / numBuckets) + 1;
    const buckets = Array.from({ length: numBuckets }, () => []);
    for (const x of data) {
        let b = Math.floor(x / width);
        if (b >= numBuckets) b = numBuckets - 1;
        buckets[b].push(x);
    }
    const out = [];
    for (const bk of buckets) {
        bk.sort((a, b) => a - b); // each bucket fits in memory
        for (const x of bk) out.push(x);
    }
    return out;
}

const input = [5, 1, 4, 2, 8, 1000000000, 0];
const sorted = bucketSort(input, 1000000000, 16);
console.log(sorted.join(" "));
