// Partition linked list: stable split into <k and >=k lists, then concatenate.
// Time O(n), Space O(1).
class Node {
    constructor(val) { this.val = val; this.next = null; }
}

function partition(head, k) {
    const lessDummy = new Node(0);
    const geDummy = new Node(0);
    let lt = lessDummy, ge = geDummy;
    for (let cur = head; cur; cur = cur.next) {
        if (cur.val < k) { lt.next = cur; lt = cur; }
        else { ge.next = cur; ge = cur; }
    }
    ge.next = null;
    lt.next = geDummy.next;
    return lessDummy.next;
}

function main() {
    const vals = [5, 1, 8, 0, 3];
    let head = null, tail = null;
    for (const v of vals) {
        const n = new Node(v);
        if (!head) { head = tail = n; } else { tail.next = n; tail = n; }
    }
    head = partition(head, 3);
    const parts = [];
    for (let cur = head; cur; cur = cur.next) parts.push(cur.val);
    console.log(parts.join(" -> "));
}

main();
