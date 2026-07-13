// Day 1803: Singleton holding two instances; getInstance alternates first (even call) / second (odd call).
// Lazy double-checked init; O(1) per call, O(1) space.
public class Solution {
    static class DualSingleton {
        final int id;
        private DualSingleton(int id) { this.id = id; }
        private static volatile DualSingleton first, second;
        private static long counter = 0;
        static synchronized DualSingleton getInstance() {
            if (first == null) { first = new DualSingleton(1); second = new DualSingleton(2); }
            long c = counter++;            // call count starts at 0
            return (c % 2 == 0) ? first : second;
        }
    }

    public static void main(String[] args) {
        DualSingleton prevEven = null;
        for (int i = 0; i < 4; i++) {
            DualSingleton inst = DualSingleton.getInstance();
            System.out.println("call" + i + "->" + inst.id);
            if (i % 2 == 0) {
                if (prevEven != null)
                    System.out.println("  even-call identity same: " + (prevEven == inst));
                prevEven = inst;
            }
        }
    }
}
