// Day 91: Closure-in-loop. `var` shares one binding (prints 9 ten times); use
// `let` (block-scoped, new binding per iteration) to capture each value. O(n).
const broken = [];
for (var k = 0; k < 10; k++) {
    var i = k; // var: one shared, function-scoped i (last assigned value is 9)
    broken.push(() => i);
}

const fixed = [];
for (let j = 0; j < 10; j++) fixed.push(() => j); // let: fresh binding each loop

console.log("Broken (prints 9 ten times):", broken.map(f => f()));
console.log("Fixed:", fixed.map(f => f()));
