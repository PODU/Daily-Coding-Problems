// Running median with two heaps (max-heap for lower half, min-heap for upper).
// O(log n) per element.
import java.util.*;

public class Solution {
    public static void main(String[] args){
        int[] stream = {2, 1, 5, 7, 2, 0, 5};
        PriorityQueue<Integer> lo = new PriorityQueue<>(Collections.reverseOrder()); // lower
        PriorityQueue<Integer> hi = new PriorityQueue<>();                            // upper

        StringBuilder sb = new StringBuilder();
        for(int x : stream){
            if(lo.isEmpty() || x <= lo.peek()) lo.add(x); else hi.add(x);
            if(lo.size() > hi.size() + 1) hi.add(lo.poll());
            else if(hi.size() > lo.size()) lo.add(hi.poll());

            double m = (lo.size() > hi.size()) ? lo.peek() : (lo.peek() + hi.peek()) / 2.0;
            if(Math.abs(m - Math.rint(m)) < 1e-9) sb.append((long)Math.rint(m)).append("\n");
            else sb.append(m).append("\n");
        }
        System.out.print(sb);
    }
}
