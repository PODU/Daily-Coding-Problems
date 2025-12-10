// Day 726: Singleton holding TWO instances; alternate on even/odd getInstance() calls.
// Approach: Static counter; odd call -> instance #2, even call -> instance #1.
// Time: O(1) per call, Space: O(1).
public class Solution {
    static class DualSingleton {
        private static final DualSingleton FIRST = new DualSingleton(1);
        private static final DualSingleton SECOND = new DualSingleton(2);
        private static int callCount = 0;
        private final int id;
        private DualSingleton(int id) { this.id = id; }
        int getId() { return id; }
        static synchronized DualSingleton getInstance() {
            callCount++;
            return (callCount % 2 == 0) ? FIRST : SECOND; // even -> first, odd -> second
        }
    }

    public static void main(String[] args) {
        for (int i = 1; i <= 4; i++)
            System.out.println("Call " + i + ": instance " + DualSingleton.getInstance().getId());
    }
}
