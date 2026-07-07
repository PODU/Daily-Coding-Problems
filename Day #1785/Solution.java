// Build origin->sorted destinations; DFS backtracking trying lexicographically
// smallest dest first; first path using all flights is the answer.
// Time O(E!) worst case, Space O(E).
import java.util.*;

public class Solution {
    static Map<String, List<String>> graph;
    static boolean[] used;
    static int total;
    static List<String[]> flightList;

    static boolean dfs(String node, List<String> path, Map<String,List<int[]>> adj) {
        if (path.size() == total + 1) return true;
        List<int[]> dests = adj.getOrDefault(node, Collections.emptyList());
        for (int[] e : dests) {
            int idx = e[0];
            if (used[idx]) continue;
            used[idx] = true;
            path.add(flightList.get(idx)[1]);
            if (dfs(flightList.get(idx)[1], path, adj)) return true;
            path.remove(path.size() - 1);
            used[idx] = false;
        }
        return false;
    }

    static List<String> findItinerary(String[][] flights, String start) {
        flightList = new ArrayList<>(Arrays.asList(flights));
        total = flights.length;
        used = new boolean[total];
        Map<String,List<int[]>> adj = new HashMap<>();
        Integer[] order = new Integer[total];
        for (int i = 0; i < total; i++) order[i] = i;
        Arrays.sort(order, (a,b) -> flights[a][1].compareTo(flights[b][1]));
        for (int i : order)
            adj.computeIfAbsent(flights[i][0], k -> new ArrayList<>()).add(new int[]{i});
        List<String> path = new ArrayList<>();
        path.add(start);
        if (dfs(start, path, adj)) return path;
        return null;
    }

    static void printResult(List<String> r) {
        if (r == null) { System.out.println("null"); return; }
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < r.size(); i++) {
            sb.append("'").append(r.get(i)).append("'");
            if (i + 1 < r.size()) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb);
    }

    public static void main(String[] args) {
        printResult(findItinerary(new String[][]{{"SFO","HKO"},{"YYZ","SFO"},{"YUL","YYZ"},{"HKO","ORD"}}, "YUL"));
        printResult(findItinerary(new String[][]{{"SFO","COM"},{"COM","YYZ"}}, "COM"));
        printResult(findItinerary(new String[][]{{"A","B"},{"A","C"},{"B","C"},{"C","A"}}, "A"));
    }
}
