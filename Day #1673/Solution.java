// Day 1673: Min wheel rotations from "000" to target avoiding dead ends.
// BFS over <=1000 states, each with 6 neighbors. Time O(1000), Space O(1000).
import java.util.*;

public class Solution {
    static Integer openLock(String target, List<String> deadends) {
        Set<String> dead = new HashSet<>(deadends);
        String start = "000";
        if (dead.contains(start) || dead.contains(target)) return null;
        if (start.equals(target)) return 0;
        Queue<String> q = new ArrayDeque<>();
        Map<String,Integer> dist = new HashMap<>();
        q.add(start); dist.put(start, 0);
        while (!q.isEmpty()) {
            String cur = q.poll();
            char[] c = cur.toCharArray();
            for (int i = 0; i < 3; i++) {
                for (int dir : new int[]{-1, 1}) {
                    char[] n = c.clone();
                    n[i] = (char) ('0' + ((c[i] - '0' + dir + 10) % 10));
                    String nxt = new String(n);
                    if (dead.contains(nxt) || dist.containsKey(nxt)) continue;
                    dist.put(nxt, dist.get(cur) + 1);
                    if (nxt.equals(target)) return dist.get(nxt);
                    q.add(nxt);
                }
            }
        }
        return null;
    }

    public static void main(String[] args) {
        Integer r = openLock("345", Arrays.asList("333")); // 12
        System.out.println(r == null ? "None" : r);
    }
}
