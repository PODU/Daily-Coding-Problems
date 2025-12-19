// Day 762: Smallest distance (words in between) between two target words.
// Single pass tracking last seen index of each word. Time: O(n), Space: O(1).
"use strict";

function smallestDistance(words, a, b) {
    let lastA = -1, lastB = -1, bestGap = Infinity;
    for (let i = 0; i < words.length; i++) {
        if (words[i] === a) {
            lastA = i;
            if (lastB !== -1) bestGap = Math.min(bestGap, lastA - lastB);
        }
        if (words[i] === b) {
            lastB = i;
            if (lastA !== -1) bestGap = Math.min(bestGap, lastB - lastA);
        }
    }
    if (bestGap === Infinity) return -1;
    return bestGap - 1;
}

const text = "dog cat hello cat dog dog hello cat world";
console.log(smallestDistance(text.split(" "), "hello", "world"));  // 1
