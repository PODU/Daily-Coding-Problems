// Max Stack: main stack + auxiliary stack holding running max. All ops O(1).
// Time O(1) per op; Space O(n).
import java.util.*;

public class Solution {
    static class MaxStack {
        private final Deque<Integer> data = new ArrayDeque<>();
        private final Deque<Integer> maxes = new ArrayDeque<>();

        void push(int v) {
            data.push(v);
            maxes.push(maxes.isEmpty() ? v : Math.max(v, maxes.peek()));
        }

        Integer pop() {
            if (data.isEmpty()) return null;
            maxes.pop();
            return data.pop();
        }

        Integer max() {
            return maxes.isEmpty() ? null : maxes.peek();
        }
    }

    public static void main(String[] args) {
        MaxStack s = new MaxStack();
        int[] vals = {3, 1, 5, 4};
        for (int v : vals) {
            s.push(v);
            System.out.println("push " + v + " -> max=" + s.max());
        }
        System.out.println("pop -> " + s.pop() + ", max=" + s.max());
        System.out.println("pop -> " + s.pop() + ", max=" + s.max());
    }
}
