// Day 1803: Singleton holding two instances; getInstance alternates first (even call) / second (odd call).
// Lazy module-scoped init; O(1) per call, O(1) space.
const DualSingleton = (() => {
  let first = null, second = null, counter = 0;
  class Inst { constructor(id) { this.id = id; } }
  return {
    getInstance() {
      if (first === null) { first = new Inst(1); second = new Inst(2); }
      const c = counter++;            // call count starts at 0
      return (c % 2 === 0) ? first : second;
    }
  };
})();

function main() {
  let prevEven = null;
  for (let i = 0; i < 4; i++) {
    const inst = DualSingleton.getInstance();
    console.log(`call${i}->${inst.id}`);
    if (i % 2 === 0) {
      if (prevEven !== null) console.log(`  even-call identity same: ${prevEven === inst}`);
      prevEven = inst;
    }
  }
}

main();
