// Max stack with O(1) push/pop/max using an auxiliary stack of running maxima.
// All operations O(1) time; O(n) space.
import java.util.ArrayDeque;
import java.util.Deque;

public class Solution {
    static class MaxStack {
        private final Deque<Integer> data = new ArrayDeque<>();
        private final Deque<Integer> maxs = new ArrayDeque<>();

        void push(int val) {
            data.push(val);
            if (maxs.isEmpty() || val >= maxs.peek()) maxs.push(val);
            else maxs.push(maxs.peek());
        }
        Integer pop() {
            if (data.isEmpty()) return null;
            maxs.pop();
            return data.pop();
        }
        Integer max() {
            return maxs.isEmpty() ? null : maxs.peek();
        }
    }

    public static void main(String[] args) {
        MaxStack s = new MaxStack();
        for (int v : new int[]{1, 5, 3, 9, 2}) s.push(v);
        System.out.println("max: " + s.max());
        System.out.println("pop: " + s.pop());
        System.out.println("pop: " + s.pop());
        System.out.println("max: " + s.max());
    }
}
