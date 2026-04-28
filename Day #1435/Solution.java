// Day 1435: Singleton holding two instances; even call -> first, odd call -> second.
// Approach: Two static instances + static call counter, return by parity of count.
// Time: O(1) per call, Space: O(1).
public class Solution {
    static class DualSingleton {
        private final int id;
        private static int counter = 0;
        private static final DualSingleton FIRST = new DualSingleton(1);
        private static final DualSingleton SECOND = new DualSingleton(2);

        private DualSingleton(int id) { this.id = id; }
        int getId() { return id; }

        static synchronized DualSingleton getInstance() {
            int n = counter++;            // 0-indexed call number
            return (n % 2 == 0) ? FIRST : SECOND;
        }
    }

    public static void main(String[] args) {
        for (int i = 0; i < 4; i++) {
            DualSingleton inst = DualSingleton.getInstance();
            System.out.println("Call " + i + " -> instance " + inst.getId());
        }
        // Even calls (0,2) -> instance 1, odd calls (1,3) -> instance 2
    }
}
