// Smallest window to sort: scan left->right tracking max for right bound,
// right->left tracking min for left bound. Time O(n), Space O(1).
function findUnsortedWindow(a) {
    const n = a.length;
    let right = -1, runMax = a[0];
    for (let i = 1; i < n; i++) {
        if (a[i] < runMax) right = i;
        else runMax = a[i];
    }
    let left = -1, runMin = a[n - 1];
    for (let i = n - 2; i >= 0; i--) {
        if (a[i] > runMin) left = i;
        else runMin = a[i];
    }
    return [left, right];
}

function main() {
    const arr = [3, 7, 5, 6, 9];
    const [l, r] = findUnsortedWindow(arr);
    console.log(`(${l}, ${r})`);
}

main();
