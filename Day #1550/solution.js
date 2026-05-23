// Closure capture demo: "buggy" closures share one `var` (prints 9 x10);
// "fixed" closures capture per-iteration value (prints 0..9). Time O(n), Space O(n).

function main() {
  // Buggy: function-scoped `var` is shared by all closures; last assigned 9.
  const buggy = [];
  for (var k = 0; k < 10; k++) {
    var i = k;
    buggy.push(() => i);
  }
  for (const f of buggy) console.log(f());

  console.log("");

  // Fixed: block-scoped `let` gives each iteration its own binding.
  const fixed = [];
  for (let j = 0; j < 10; j++) {
    fixed.push(() => j);
  }
  for (const f of fixed) console.log(f());
}

main();
