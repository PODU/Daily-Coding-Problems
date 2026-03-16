// Day 1211: Interleave first half of a stack with the reversed second half, using one queue.
// Load stack bottom->top into the queue, then alternately push front/back (back via rotation). Time O(n^2), Space O(n).
import java.util.*;

public class Solution {
    static void interleave(Deque<Integer> st) {
        Queue<Integer> q = new LinkedList<>();
        int n = 0;
        while (!st.isEmpty()) { q.add(st.pop()); n++; }   // front = old top
        for (int i = 0; i < n; i++) st.push(q.poll());     // top = old bottom
        for (int i = 0; i < n; i++) q.add(st.pop());       // front = bottom
        int remaining = n;
        boolean takeFront = true;
        while (remaining > 0) {
            if (takeFront) st.push(q.poll());
            else {
                for (int i = 0; i < remaining - 1; i++) q.add(q.poll());
                st.push(q.poll());
            }
            remaining--; takeFront = !takeFront;
        }
    }

    public static void main(String[] args) {
        Deque<Integer> st = new ArrayDeque<>();
        for (int x : new int[]{1, 2, 3, 4, 5}) st.push(x); // bottom -> top
        interleave(st);
        List<Integer> out = new ArrayList<>();
        while (!st.isEmpty()) out.add(st.pop());
        Collections.reverse(out); // bottom -> top
        System.out.println(out); // [1, 5, 2, 4, 3]
    }
}
