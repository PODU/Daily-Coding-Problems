// Day 1116 - Quack: push/pop left, pull right, using three stacks.
// Two stacks L (top=leftmost) and R (top=rightmost); rebalance by splitting the
// other in half via scratch stack T. Amortized O(1) per op, O(1) extra memory.
import java.util.*;

public class Solution {
    static class Quack {
        Deque<Integer> L = new ArrayDeque<>();
        Deque<Integer> R = new ArrayDeque<>();
        Deque<Integer> T = new ArrayDeque<>();

        void push(int x) { L.push(x); }

        void rebalanceToL() {
            int m = R.size();
            int rightCount = m / 2;
            for (int i = 0; i < rightCount; i++) T.push(R.pop());
            while (!R.isEmpty()) L.push(R.pop());
            while (!T.isEmpty()) R.push(T.pop());
        }

        void rebalanceToR() {
            int m = L.size();
            int leftCount = m / 2;
            for (int i = 0; i < leftCount; i++) T.push(L.pop());
            while (!L.isEmpty()) R.push(L.pop());
            while (!T.isEmpty()) L.push(T.pop());
        }

        int pop() {
            if (L.isEmpty()) rebalanceToL();
            return L.pop();
        }

        int pull() {
            if (R.isEmpty()) rebalanceToR();
            return R.pop();
        }
    }

    public static void main(String[] args) {
        Quack q = new Quack();
        for (int x : new int[]{1, 2, 3}) q.push(x);
        System.out.println("pop: " + q.pop());   // 3
        System.out.println("pull: " + q.pull()); // 1
        for (int x : new int[]{4, 5}) q.push(x);
        System.out.println("pull: " + q.pull()); // 2
        System.out.println("pop: " + q.pop());   // 5
        System.out.println("pull: " + q.pull()); // 4
    }
}
