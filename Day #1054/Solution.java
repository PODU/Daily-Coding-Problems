// Day 1054: Dual-instance singleton. Holds two lazily-created instances and an
// alternating counter; getInstance() returns instance[count % 2], then bumps the
// counter. Time O(1) per call, Space O(1).

public class Solution {
    static class DualSingleton {
        final int id;
        private static DualSingleton[] instances;
        private static int count = 0;

        private DualSingleton(int id) { this.id = id; }

        static synchronized DualSingleton getInstance() {
            if (instances == null)
                instances = new DualSingleton[]{new DualSingleton(1), new DualSingleton(2)};
            DualSingleton chosen = instances[count % 2];
            count++;
            return chosen;
        }
    }

    public static void main(String[] args) {
        for (int call = 0; call < 6; call++) {
            String kind = (call % 2 == 0) ? "first" : "second";
            String parity = (call % 2 == 0) ? "even" : "odd";
            System.out.println("call " + call + " (" + parity + ") -> " + kind
                + " instance (id=" + DualSingleton.getInstance().id + ")");
        }
    }
}
