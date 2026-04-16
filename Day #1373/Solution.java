// 2D iterator with lazy outer/inner pointers (no flatten/clone).
// next() & hasNext() amortized O(1), Space O(1) extra.
import java.util.*;

public class Solution {
    static class Iterator2D {
        private final int[][] data;
        private int outer = 0, inner = 0;
        Iterator2D(int[][] data) { this.data = data; }
        boolean hasNext() {
            while (outer < data.length && inner >= data[outer].length) { outer++; inner = 0; }
            return outer < data.length;
        }
        int next() {
            if (!hasNext()) throw new NoSuchElementException("no more elements");
            return data[outer][inner++];
        }
    }

    public static void main(String[] args) {
        int[][] data = {{1, 2}, {3}, {}, {4, 5, 6}};
        Iterator2D it = new Iterator2D(data);
        StringBuilder sb = new StringBuilder();
        while (it.hasNext()) {
            if (sb.length() > 0) sb.append(", ");
            sb.append(it.next());
        }
        System.out.println(sb);
    }
}
