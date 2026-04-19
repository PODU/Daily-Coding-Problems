// FIFO queue via two stacks (in/out); amortized O(1) per op, O(n) space.
'use strict';

class MyQueue {
    constructor() { this.in = []; this.out = []; }
    enqueue(x) { this.in.push(x); }
    dequeue() {
        if (this.out.length === 0)
            while (this.in.length) this.out.push(this.in.pop());
        return this.out.pop();
    }
}

function main() {
    const q = new MyQueue();
    q.enqueue(1); q.enqueue(2); q.enqueue(3);
    console.log(q.dequeue()); // 1
    q.enqueue(4);
    console.log(q.dequeue()); // 2
    console.log(q.dequeue()); // 3
    console.log(q.dequeue()); // 4
}

main();
