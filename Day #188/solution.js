// Day 188: Closure-in-a-loop late binding (JS analog of the Python snippet).
// `var` is function-scoped, so all closures share one i and read 3 after the loop.
// Fix: `let` gives a fresh binding per iteration. Time/Space O(n).
function makeFunctionsBuggy() {
  const funcs = [];
  for (var idx = 0; idx < 3; idx++) {
    var i = [1, 2, 3][idx];
    funcs.push(function () { return i; });
  }
  return funcs;
}

function makeFunctionsFixed() {
  const funcs = [];
  for (let idx = 0; idx < 3; idx++) {
    let i = [1, 2, 3][idx];
    funcs.push(function () { return i; });
  }
  return funcs;
}

console.log("Late binding prints:");
for (const f of makeFunctionsBuggy()) console.log(f());
console.log("Fixed prints:");
for (const f of makeFunctionsFixed()) console.log(f());
