// Day 367: Lazily merge two sorted iterators into one sorted iterator.
// A generator peeks the head of each iterator and yields the smaller; nothing is
// buffered into memory. Time O(n+m), Space O(1).
'use strict';

function* mergeIterators(a, b) {
  const ia = a[Symbol.iterator]();
  const ib = b[Symbol.iterator]();
  let x = ia.next();
  let y = ib.next();
  while (!x.done || !y.done) {
    if (y.done || (!x.done && x.value <= y.value)) {
      yield x.value;
      x = ia.next();
    } else {
      yield y.value;
      y = ib.next();
    }
  }
}

const foo = [5, 10, 15];
const bar = [3, 8, 9];
for (const num of mergeIterators(foo, bar)) console.log(num); // 3 5 8 9 10 15
