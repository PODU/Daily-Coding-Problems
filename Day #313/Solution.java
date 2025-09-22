// Day 313: Min moves on a 3-wheel lock from 000 to target, avoiding dead ends.
// BFS over <=1000 states. O(1000) time. Prints -1 to represent None.
import java.util.*;
public class Solution {
    static int openLock(String target, String[] deadends) {
        Set<String> dead = new HashSet<>(Arrays.asList(deadends));
        String start = "000";
        if (dead.contains(start)) return -1;
        if (start.equals(target)) return 0;
        Queue<String> q = new ArrayDeque<>();
        Map<String, Integer> dist = new HashMap<>();
        q.add(start); dist.put(start, 0);
        while (!q.isEmpty()) {
            String cur = q.poll();
            for (int i = 0; i < 3; i++) for (int d = -1; d <= 1; d += 2) {
                char[] arr = cur.toCharArray();
                arr[i] = (char) ('0' + ((arr[i] - '0' + d + 10) % 10));
                String nx = new String(arr);
                if (dead.contains(nx) || dist.containsKey(nx)) continue;
                dist.put(nx, dist.get(cur) + 1);
                if (nx.equals(target)) return dist.get(nx);
                q.add(nx);
            }
        }
        return -1;
    }
    public static void main(String[] a) {
        System.out.println(openLock("123", new String[]{})); // 6
    }
}
