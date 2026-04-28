// Day 1435: Singleton holding two instances; even call -> first, odd call -> second.
// Approach: Two stored instances + module-level call counter, return by parity of count.
// Time: O(1) per call, Space: O(1).

class DualSingleton {
  constructor(id) { this.id = id; }

  static getInstance() {
    if (!DualSingleton._first) {
      DualSingleton._first = new DualSingleton(1);
      DualSingleton._second = new DualSingleton(2);
      DualSingleton._counter = 0;
    }
    const n = DualSingleton._counter++; // 0-indexed call number
    return n % 2 === 0 ? DualSingleton._first : DualSingleton._second;
  }
}

for (let i = 0; i < 4; i++) {
  const inst = DualSingleton.getInstance();
  console.log(`Call ${i} -> instance ${inst.id}`);
}
// Even calls (0,2) -> instance 1, odd calls (1,3) -> instance 2
