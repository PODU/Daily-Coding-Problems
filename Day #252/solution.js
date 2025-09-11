// Egyptian fraction via greedy: repeatedly take ceil(b/a) as next unit denominator.
// Time: O(number of terms) iterations; Space: O(terms). a/b proper (a<b).

function egyptian(a, b) {
    const denoms = [];
    while (a !== 0) {
        const d = Math.ceil(b / a);
        denoms.push(d);
        a = a * d - b;
        b = b * d;
    }
    return denoms;
}

const denoms = egyptian(4, 13);
console.log(denoms.map(d => `1 / ${d}`).join(" + "));
