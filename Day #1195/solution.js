// Smallest window containing all distinct chars of the string. Sliding window:
// expand right, shrink left while all distinct kinds present. O(N) time, O(K) space.

function smallestWindow(s) {
    const distinct = new Set(s).size;
    const cnt = new Map();
    let have = 0, left = 0, best = Infinity;
    for (let right = 0; right < s.length; right++) {
        const c = s[right];
        cnt.set(c, (cnt.get(c) || 0) + 1);
        if (cnt.get(c) === 1) have++;
        while (have === distinct) {
            best = Math.min(best, right - left + 1);
            const lc = s[left];
            cnt.set(lc, cnt.get(lc) - 1);
            if (cnt.get(lc) === 0) have--;
            left++;
        }
    }
    return best === Infinity ? 0 : best;
}

function main() {
    console.log(smallestWindow("jiujitsu"));
}

main();
