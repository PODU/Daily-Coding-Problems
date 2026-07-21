// Day 1853: Stack with O(1) push/pop/max.
// Keep an auxiliary stack of running maxima alongside the data stack. All ops O(1) time, O(n) space.
import java.util.*;

public class Solution {
    static class MaxStack {
        private final Deque<Integer> data = new ArrayDeque<>();
        private final Deque<Integer> maxs = new ArrayDeque<>();

        void push(int v) {
            data.push(v);
            maxs.push(maxs.isEmpty() ? v : Math.max(maxs.peek(), v));
        }
        int pop() {
            if (data.isEmpty()) throw new RuntimeException("empty stack");
            maxs.pop();
            return data.pop();
        }
        int max() {
            if (maxs.isEmpty()) throw new RuntimeException("empty stack");
            return maxs.peek();
        }
    }

    public static void main(String[] args) {
        MaxStack s = new MaxStack();
        s.push(1); s.push(5); s.push(3);
        System.out.println(s.max()); // 5
        System.out.println(s.pop()); // 3
        System.out.println(s.max()); // 5
        System.out.println(s.pop()); // 5
        System.out.println(s.max()); // 1
    }
}
