// Day 455: Conway's Game of Life on an infinite board.
// Hash set of live cells; tally neighbours each tick. Time O(live) per step.
import java.util.HashMap;
import java.util.HashSet;
import java.util.Map;
import java.util.Set;

public class Solution {
    static long key(int r, int c) { return (((long) r) << 32) ^ (c & 0xffffffffL); }
    static int row(long k) { return (int) (k >> 32); }
    static int col(long k) { return (int) k; }

    static Set<Long> step(Set<Long> live) {
        Map<Long, Integer> cnt = new HashMap<>();
        for (long c : live) {
            int r = row(c), cc = col(c);
            for (int dr = -1; dr <= 1; dr++)
                for (int dc = -1; dc <= 1; dc++)
                    if (dr != 0 || dc != 0)
                        cnt.merge(key(r + dr, cc + dc), 1, Integer::sum);
        }
        Set<Long> next = new HashSet<>();
        for (Map.Entry<Long, Integer> e : cnt.entrySet()) {
            int n = e.getValue();
            boolean alive = live.contains(e.getKey());
            if (n == 3 || (alive && n == 2)) next.add(e.getKey());
        }
        return next;
    }

    static void printBoard(Set<Long> live) {
        if (live.isEmpty()) { System.out.println("(empty)"); return; }
        int r0 = Integer.MAX_VALUE, r1 = Integer.MIN_VALUE, c0 = Integer.MAX_VALUE, c1 = Integer.MIN_VALUE;
        for (long k : live) {
            r0 = Math.min(r0, row(k)); r1 = Math.max(r1, row(k));
            c0 = Math.min(c0, col(k)); c1 = Math.max(c1, col(k));
        }
        StringBuilder sb = new StringBuilder();
        for (int r = r0; r <= r1; r++) {
            for (int c = c0; c <= c1; c++)
                sb.append(live.contains(key(r, c)) ? '*' : '.');
            sb.append('\n');
        }
        System.out.print(sb);
    }

    public static void main(String[] args) {
        Set<Long> live = new HashSet<>();
        live.add(key(1, 0)); live.add(key(1, 1)); live.add(key(1, 2));
        int steps = 2;
        System.out.println("Step 0:"); printBoard(live);
        for (int s = 1; s <= steps; s++) {
            live = step(live);
            System.out.println("Step " + s + ":");
            printBoard(live);
        }
    }
}
