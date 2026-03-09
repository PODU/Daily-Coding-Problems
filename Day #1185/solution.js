// Day 1185: Smallest distance (words in between) between two words in a text.
// Single pass tracking last index of each target word; min |i-j| - 1.
// Time O(N), Space O(1).

function shortestDistance(text, w1, w2) {
    const words = text.split(/\s+/);
    let p1 = -1, p2 = -1, best = Infinity;
    for (let i = 0; i < words.length; i++) {
        if (words[i] === w1) p1 = i;
        if (words[i] === w2) p2 = i;
        if (p1 >= 0 && p2 >= 0) best = Math.min(best, Math.abs(p1 - p2));
    }
    return best === Infinity ? -1 : best - 1;
}

const text = "dog cat hello cat dog dog hello cat world";
console.log(shortestDistance(text, "hello", "world")); // 1
