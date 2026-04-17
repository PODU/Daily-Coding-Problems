// Conway's Game of Life on an infinite board using a set of live coordinates.
// Each step counts neighbours only around live cells.
// Time: O(L) per step, Space: O(L).
import java.util.*;

public class Solution {
    static long key(int x, int y) { return ((long) x << 32) ^ (y & 0xffffffffL); }
    static int kx(long k) { return (int) (k >> 32); }
    static int ky(long k) { return (int) k; }

    static Set<Long> step(Set<Long> live) {
        Map<Long, Integer> cnt = new HashMap<>();
        for (long c : live) {
            int x = kx(c), y = ky(c);
            for (int dx = -1; dx <= 1; dx++)
                for (int dy = -1; dy <= 1; dy++)
                    if (dx != 0 || dy != 0)
                        cnt.merge(key(x + dx, y + dy), 1, Integer::sum);
        }
        Set<Long> nb = new HashSet<>();
        for (Map.Entry<Long, Integer> e : cnt.entrySet()) {
            int c = e.getValue();
            if (c == 3 || (c == 2 && live.contains(e.getKey()))) nb.add(e.getKey());
        }
        return nb;
    }

    static void render(Set<Long> live) {
        if (live.isEmpty()) { System.out.println("(empty)"); return; }
        int minx = Integer.MAX_VALUE, maxx = Integer.MIN_VALUE, miny = Integer.MAX_VALUE, maxy = Integer.MIN_VALUE;
        for (long c : live) {
            minx = Math.min(minx, kx(c)); maxx = Math.max(maxx, kx(c));
            miny = Math.min(miny, ky(c)); maxy = Math.max(maxy, ky(c));
        }
        for (int x = minx; x <= maxx; x++) {
            StringBuilder row = new StringBuilder();
            for (int y = miny; y <= maxy; y++)
                row.append(live.contains(key(x, y)) ? '*' : '.');
            System.out.println(row);
        }
    }

    public static void main(String[] args) {
        Set<Long> live = new HashSet<>(Arrays.asList(key(1, 0), key(1, 1), key(1, 2)));
        int steps = 2;
        for (int i = 0; i <= steps; i++) {
            System.out.println("Step " + i + ":");
            render(live);
            System.out.println();
            live = step(live);
        }
    }
}
