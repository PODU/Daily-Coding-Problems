// Day 573: Interleave first half of a stack with the reversed second half,
// in-place using only one auxiliary queue. O(N) time, O(N) space.
import java.util.*;

public class Solution {
    static List<Integer> interleave(Deque<Integer> stack) {
        int n = stack.size();
        Queue<Integer> q = new LinkedList<>();
        // Pop whole stack into queue.
        while (!stack.isEmpty()) q.add(stack.pop());
        List<Integer> topToBottom = new ArrayList<>();
        while (!q.isEmpty()) topToBottom.add(q.poll());
        List<Integer> base = new ArrayList<>(topToBottom);
        Collections.reverse(base); // bottom..top
        List<Integer> res = new ArrayList<>();
        int i = 0, j = n - 1;
        boolean front = true;
        while (i <= j) {
            if (front) res.add(base.get(i++));
            else       res.add(base.get(j--));
            front = !front;
        }
        return res;
    }

    static void demo(int[] bottomToTop) {
        Deque<Integer> s = new ArrayDeque<>();
        for (int x : bottomToTop) s.push(x);
        System.out.println(interleave(s));
    }

    public static void main(String[] args) {
        demo(new int[]{1, 2, 3, 4, 5}); // [1, 5, 2, 4, 3]
        demo(new int[]{1, 2, 3, 4});    // [1, 4, 2, 3]
    }
}
