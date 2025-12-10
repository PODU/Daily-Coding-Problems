// Day 726: Singleton holding TWO instances; alternate on even/odd getInstance() calls.
// Approach: Module counter; odd call -> instance #2, even call -> instance #1.
// Time: O(1) per call, Space: O(1).

class DualSingleton {
  constructor(id) { this.id = id; }
  static getInstance() {
    if (!DualSingleton._first) {
      DualSingleton._first = new DualSingleton(1);
      DualSingleton._second = new DualSingleton(2);
      DualSingleton._count = 0;
    }
    DualSingleton._count++;
    return DualSingleton._count % 2 === 0 ? DualSingleton._first : DualSingleton._second;
  }
}

for (let i = 1; i <= 4; i++)
  console.log(`Call ${i}: instance ${DualSingleton.getInstance().id}`);
