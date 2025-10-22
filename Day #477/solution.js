// Closure capture: `var` is function-scoped, so all closures share one i and read its final
// assigned value (9). Fix with block-scoped `let i`, giving each iteration its own binding
// -> prints 0..9. (Python analogue: lambda: i prints 9 ten times; fix is lambda i=i: i.)
function buggy() {
  const fns = [];
  for (var k = 0; k < 10; k++) {
    var i = k; // one shared, function-scoped `i`; last assigned value is 9
    fns.push(function () { return i; });
  }
  return fns;
}

function fixed() {
  const fns = [];
  for (let i = 0; i < 10; i++) {
    fns.push(function () { return i; }); // per-iteration `let i`
  }
  return fns;
}

console.log("Buggy:");
buggy().forEach((f) => console.log(f()));
console.log("Fixed:");
fixed().forEach((f) => console.log(f()));
