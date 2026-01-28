// Conway's Game of Life on an infinite board: store live cells in a set; each step
// tally neighbor counts only around live cells, then apply the 4 rules.
// Time: O(L) per step (L live cells); Space: O(L). Printing bounds to the live region.
import java.util.*;

public class Solution {
    private Set<Long> live = new HashSet<>();

    // encode (row, col) into a single long key (offset to keep non-negative-ish packing).
    private static long key(int r, int c) {
        return (((long) r) << 32) ^ (c & 0xffffffffL);
    }
    private static int rowOf(long k) { return (int) (k >> 32); }
    private static int colOf(long k) { return (int) k; }

    public Solution(int[][] cells) {
        for (int[] cell : cells) live.add(key(cell[0], cell[1]));
    }

    public void step() {
        Map<Long, Integer> counts = new HashMap<>();
        for (long k : live) {
            int r = rowOf(k), c = colOf(k);
            for (int dr = -1; dr <= 1; dr++)
                for (int dc = -1; dc <= 1; dc++)
                    if (dr != 0 || dc != 0)
                        counts.merge(key(r + dr, c + dc), 1, Integer::sum);
        }
        Set<Long> next = new HashSet<>();
        for (Map.Entry<Long, Integer> e : counts.entrySet()) {
            int cnt = e.getValue();
            boolean alive = live.contains(e.getKey());
            if (cnt == 3 || (alive && cnt == 2)) next.add(e.getKey());
        }
        live = next;
    }

    public void print(int stepNo) {
        System.out.println("Step " + stepNo + ":");
        if (live.isEmpty()) { System.out.println("(empty)\n"); return; }
        int minR = Integer.MAX_VALUE, maxR = Integer.MIN_VALUE;
        int minC = Integer.MAX_VALUE, maxC = Integer.MIN_VALUE;
        for (long k : live) {
            int r = rowOf(k), c = colOf(k);
            minR = Math.min(minR, r); maxR = Math.max(maxR, r);
            minC = Math.min(minC, c); maxC = Math.max(maxC, c);
        }
        StringBuilder sb = new StringBuilder();
        for (int r = minR; r <= maxR; r++) {
            for (int c = minC; c <= maxC; c++)
                sb.append(live.contains(key(r, c)) ? '*' : '.');
            sb.append('\n');
        }
        System.out.println(sb);
    }

    public static void main(String[] args) {
        Solution g = new Solution(new int[][]{{0, 0}, {0, 1}, {0, 2}}); // horizontal blinker
        int steps = 2;
        g.print(0);
        for (int s = 1; s <= steps; s++) {
            g.step();
            g.print(s);
        }
    }
}
