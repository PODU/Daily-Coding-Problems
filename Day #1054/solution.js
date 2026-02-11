// Day 1054: Dual-instance singleton. Holds two lazily-created instances and an
// alternating counter; getInstance() returns instance[count % 2], then bumps the
// counter. Time O(1) per call, Space O(1).

class DualSingleton {
  constructor(id) { this.id = id; }

  static getInstance() {
    if (!DualSingleton._instances) {
      DualSingleton._instances = [new DualSingleton(1), new DualSingleton(2)];
      DualSingleton._count = 0;
    }
    const inst = DualSingleton._instances[DualSingleton._count % 2];
    DualSingleton._count++;
    return inst;
  }
}

for (let call = 0; call < 6; call++) {
  const kind = call % 2 === 0 ? "first" : "second";
  const parity = call % 2 === 0 ? "even" : "odd";
  console.log(`call ${call} (${parity}) -> ${kind} instance (id=${DualSingleton.getInstance().id})`);
}
