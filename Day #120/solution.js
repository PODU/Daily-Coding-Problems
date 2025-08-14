// Day 120: Dual singleton. Two fixed instances; odd call -> 2nd, even call -> 1st.
// getInstance() is O(1); instances created once and cached on the class.
class DualSingleton {
  constructor(name) {
    this.name = name;
  }
  static getInstance() {
    if (!DualSingleton._first) {
      DualSingleton._first = new DualSingleton("first");
      DualSingleton._second = new DualSingleton("second");
      DualSingleton._count = 0;
    }
    DualSingleton._count++; // 1-based call number
    return DualSingleton._count % 2 === 0
      ? DualSingleton._first   // even -> first
      : DualSingleton._second; // odd  -> second
  }
}

for (let i = 0; i < 4; i++) {
  console.log(DualSingleton.getInstance().name); // second, first, second, first
}
