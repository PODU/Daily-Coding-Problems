// Custom left fold (reduce). acc=init; for each x: acc=f(acc,x). O(n) time, O(1) space.
'use strict';

function myReduce(arr, f, init){
    let acc = init;
    for(const x of arr) acc = f(acc, x);
    return acc;
}

function add(a, b){ return a + b; }

function sum(lst){
    return myReduce(lst, add, 0);
}

function main(){
    console.log(sum([1, 2, 3, 4, 5]));
}

main();
