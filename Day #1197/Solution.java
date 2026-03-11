// Max stack with O(1) push/pop/max.
// Auxiliary stack stores running maxima alongside main stack. All ops O(1); space O(N).
import java.util.*;

public class Solution {
    static class MaxStack {
        private final Deque<Integer> data = new ArrayDeque<>();
        private final Deque<Integer> maxs = new ArrayDeque<>();

        void push(int val) {
            data.push(val);
            maxs.push(maxs.isEmpty() ? val : Math.max(val, maxs.peek()));
        }
        // returns top, or null if empty
        Integer pop() {
            if (data.isEmpty()) return null;
            maxs.pop();
            return data.pop();
        }
        Integer max() {
            return maxs.isEmpty() ? null : maxs.peek();
        }
    }

    static String show(Integer v) { return v == null ? "null" : v.toString(); }

    public static void main(String[] args) {
        MaxStack s = new MaxStack();
        s.push(3); s.push(1); s.push(5); s.push(2);
        System.out.println("max: " + show(s.max()));
        System.out.println("pop: " + show(s.pop()));
        System.out.println("pop: " + show(s.pop()));
        System.out.println("max: " + show(s.max()));
    }
}
