// Day 575: 2D iterator over an array of arrays without flattening/cloning.
// Track (row, col); advance over empty rows. next/hasNext amortized O(1).
import java.util.*;

public class Solution {
    static class Iterator2D {
        private final int[][] data;
        private int row = 0, col = 0;
        Iterator2D(int[][] d) { this.data = d; skipEmpty(); }
        private void skipEmpty() {
            while (row < data.length && col >= data[row].length) { row++; col = 0; }
        }
        boolean hasNext() { return row < data.length; }
        int next() {
            if (!hasNext()) throw new NoSuchElementException();
            int v = data[row][col++];
            skipEmpty();
            return v;
        }
    }

    public static void main(String[] args) {
        int[][] arr = {{1, 2}, {3}, {}, {4, 5, 6}};
        Iterator2D it = new Iterator2D(arr);
        StringBuilder sb = new StringBuilder();
        while (it.hasNext()) {
            if (sb.length() > 0) sb.append(", ");
            sb.append(it.next());
        }
        System.out.println(sb); // 1, 2, 3, 4, 5, 6
    }
}
