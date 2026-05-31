// Day 1584: 2D iterator over array of arrays (no flatten/clone).
// Maintain (outer,inner) indices; skip() advances past empty inner arrays. Time: O(1) amortized; Space: O(1).
import java.util.*;

public class Solution {
    static class Iterator2D {
        private final int[][] data;
        private int outer = 0, inner = 0;
        Iterator2D(int[][] d) { data = d; skip(); }
        private void skip() { while (outer < data.length && inner >= data[outer].length) { outer++; inner = 0; } }
        boolean hasNext() { skip(); return outer < data.length; }
        int next() {
            if (!hasNext()) throw new NoSuchElementException("no more elements");
            return data[outer][inner++];
        }
    }

    public static void main(String[] args) {
        int[][] v = {{1, 2}, {3}, {}, {4, 5, 6}};
        Iterator2D it = new Iterator2D(v);
        List<Integer> out = new ArrayList<>();
        while (it.hasNext()) out.add(it.next());
        System.out.println(String.join(", ", out.stream().map(String::valueOf).toArray(String[]::new)));
        // 1, 2, 3, 4, 5, 6
    }
}
