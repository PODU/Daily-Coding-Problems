// Day 436: Three stacks backed by one list using node-based singly-linked
// indices + a free list. push/pop are O(1) time, O(n) space overall.
import java.util.*;

public class Solution {
    static class ThreeStacks {
        private final ArrayList<int[]> data = new ArrayList<>(); // each node: {val, prev}
        private final int[] tops = {-1, -1, -1};
        private final ArrayDeque<Integer> freeList = new ArrayDeque<>();

        void push(int item, int s) {
            int idx;
            if (!freeList.isEmpty()) { idx = freeList.pop(); data.set(idx, new int[]{item, tops[s]}); }
            else { idx = data.size(); data.add(new int[]{item, tops[s]}); }
            tops[s] = idx;
        }

        int pop(int s) {
            if (tops[s] == -1) throw new RuntimeException("stack " + s + " is empty");
            int idx = tops[s];
            int[] node = data.get(idx);
            tops[s] = node[1];
            freeList.push(idx);
            return node[0];
        }
    }

    public static void main(String[] args) {
        ThreeStacks st = new ThreeStacks();
        st.push(1, 0); st.push(2, 0);
        st.push(10, 1);
        st.push(100, 2); st.push(200, 2);
        System.out.println(st.pop(0)); // 2
        System.out.println(st.pop(0)); // 1
        System.out.println(st.pop(1)); // 10
        System.out.println(st.pop(2)); // 200
        System.out.println(st.pop(2)); // 100
    }
}
