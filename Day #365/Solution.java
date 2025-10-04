// Day 365: "Quack" deque (push/pop left, pull right) from three stacks.
// l holds the left part (left end on top), r the right part (right end on top);
// tmp is a transient helper used only to re-split when one side empties.
// Rebalance moves k after k ops -> O(1) amortized, O(1) extra memory.
import java.util.*;

public class Solution {
    static class Quack {
        Deque<Integer> l = new ArrayDeque<>(), r = new ArrayDeque<>(), tmp = new ArrayDeque<>();

        void push(int x) { l.push(x); }

        void rebalance(Deque<Integer> to, Deque<Integer> from, int toCount) {
            int n = from.size();
            for (int i = 0; i < n - toCount; i++) tmp.push(from.pop());
            for (int i = 0; i < toCount; i++) to.push(from.pop());
            while (!tmp.isEmpty()) from.push(tmp.pop());
        }

        int pop() {
            if (l.isEmpty()) rebalance(l, r, (r.size() + 1) / 2);
            return l.pop();
        }

        int pull() {
            if (r.isEmpty()) rebalance(r, l, (l.size() + 1) / 2);
            return r.pop();
        }
    }

    public static void main(String[] args) {
        Quack q = new Quack();
        q.push(1); q.push(2); q.push(3);
        System.out.println(q.pop());   // 3
        System.out.println(q.pull());  // 1
        q.push(4); q.push(5);
        System.out.println(q.pull());  // 2
        System.out.println(q.pop());   // 5
        System.out.println(q.pop());   // 4
    }
}
