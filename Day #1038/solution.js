// Reverse word order in-place: reverse whole char array, then reverse each word.
// Time: O(n), Space: O(1) extra (on the mutable array buffer).

function reverseRange(a, i, j) {
    while (i < j) {
        const t = a[i];
        a[i] = a[j];
        a[j] = t;
        i++;
        j--;
    }
}

function reverseWords(a) {
    reverseRange(a, 0, a.length - 1);
    let start = 0;
    for (let i = 0; i <= a.length; i++) {
        if (i === a.length || a[i] === ' ') {
            reverseRange(a, start, i - 1);
            start = i + 1;
        }
    }
}

function main() {
    const a = "hello world here".split("");
    reverseWords(a);
    console.log('"' + a.join("") + '"');
}

main();
