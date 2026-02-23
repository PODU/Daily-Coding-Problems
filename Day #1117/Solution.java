// Day 1117 - Three stacks backed by a single list
// Each entry stores (value, prev_index); per-stack heads + free list give O(1)
// push/pop sharing one list. Space O(n).
import java.util.*;

public class Solution {
    static class Stack3 {
        List<int[]> list = new ArrayList<>(); // {value, prev}
        int[] heads = {-1, -1, -1};
        Deque<Integer> free = new ArrayDeque<>();

        void push(int item, int s) {
            int idx;
            if (!free.isEmpty()) {
                idx = free.pop();
                list.set(idx, new int[]{item, heads[s]});
            } else {
                idx = list.size();
                list.add(new int[]{item, heads[s]});
            }
            heads[s] = idx;
        }

        int pop(int s) {
            int idx = heads[s];
            if (idx == -1) throw new RuntimeException("pop from empty stack " + s);
            int value = list.get(idx)[0], prev = list.get(idx)[1];
            heads[s] = prev;
            free.push(idx);
            return value;
        }
    }

    public static void main(String[] args) {
        Stack3 s = new Stack3();
        s.push(1, 0); s.push(2, 0);
        s.push(3, 1);
        s.push(4, 2); s.push(5, 2);
        System.out.println("pop(0): " + s.pop(0)); // 2
        System.out.println("pop(0): " + s.pop(0)); // 1
        System.out.println("pop(2): " + s.pop(2)); // 5
        System.out.println("pop(1): " + s.pop(1)); // 3
        System.out.println("pop(2): " + s.pop(2)); // 4
    }
}
