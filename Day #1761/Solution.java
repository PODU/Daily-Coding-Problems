// Day 1761: Interleave first half of a stack with the reversed second half,
// in-place using ONE auxiliary queue (only push/pop/enqueue/dequeue).
// Dump stack into queue (reverses), rebuild taking alternately back/front.
// Time O(n^2), Space O(n). Stack modeled as a list with end = top.
import java.util.*;

public class Solution {
    static List<Integer> interleave(List<Integer> input) {
        List<Integer> stack = new ArrayList<>(input); // end = top
        Queue<Integer> q = new LinkedList<>();
        while (!stack.isEmpty()) q.add(stack.remove(stack.size() - 1)); // pop top
        boolean takeBack = true;
        while (!q.isEmpty()) {
            int v;
            if (takeBack) {
                for (int i = 0; i + 1 < q.size(); i++) q.add(q.poll());
                v = q.poll();
            } else {
                v = q.poll();
            }
            stack.add(v); // push
            takeBack = !takeBack;
        }
        return stack;
    }

    static String fmt(List<Integer> v) {
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < v.size(); i++) sb.append(v.get(i)).append(i + 1 < v.size() ? ", " : "");
        return sb.append("]").toString();
    }

    public static void main(String[] args) {
        System.out.println(fmt(interleave(Arrays.asList(1, 2, 3, 4, 5))));
        System.out.println(fmt(interleave(Arrays.asList(1, 2, 3, 4))));
    }
}
