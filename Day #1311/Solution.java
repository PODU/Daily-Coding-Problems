// Quack (deque) via three stacks. On underflow of one end, rebalance by moving
// half the elements through the third stack -> O(1) amortized, O(1) extra memory.
import java.util.*;

public class Solution {
    static class Quack {
        Deque<Integer> L = new ArrayDeque<>(), R = new ArrayDeque<>(), T = new ArrayDeque<>();

        void rebalance(Deque<Integer> src, Deque<Integer> dst, Deque<Integer> tmp) {
            int n = src.size(), k = n / 2;
            for (int i = 0; i < k; i++)     tmp.push(src.pop());
            for (int i = 0; i < n - k; i++) dst.push(src.pop());
            for (int i = 0; i < k; i++)     src.push(tmp.pop());
        }
        void push(int x) { L.push(x); }
        int pop()  { if (L.isEmpty()) rebalance(R, L, T); return L.pop(); }
        int pull() { if (R.isEmpty()) rebalance(L, R, T); return R.pop(); }
    }

    public static void main(String[] args) {
        Quack q = new Quack();
        q.push(1); q.push(2); q.push(3);
        System.out.println(q.pop());  // 3
        System.out.println(q.pull()); // 1
        q.push(4);
        System.out.println(q.pull()); // 2
        System.out.println(q.pop());  // 4
    }
}
