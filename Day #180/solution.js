// Interleave first half with reversed second half using ONE auxiliary queue.
// Result (bottom->top) = arr[0],arr[n-1],arr[1],arr[n-2],...  O(n^2) time, O(n) space.
function interleave(arr) {
    const stack = arr.slice(); // top = end (push/pop)
    const q = [];              // enqueue = push(end), dequeue = shift(front)

    while (stack.length) q.push(stack.pop());   // A: stack -> queue
    while (q.length) stack.push(q.shift());     // B: queue -> stack (reverse)
    while (stack.length) q.push(stack.pop());   // C: stack -> queue (front..back = bottom..top)

    let takeFront = true;
    while (q.length) {
        if (takeFront) {
            stack.push(q.shift());
        } else {
            for (let k = q.length - 1; k > 0; k--) q.push(q.shift()); // rotate back to front
            stack.push(q.shift());
        }
        takeFront = !takeFront;
    }
    return stack;
}

const fmt = a => "[" + a.join(", ") + "]";
console.log(fmt(interleave([1, 2, 3, 4, 5])));
console.log(fmt(interleave([1, 2, 3, 4])));
