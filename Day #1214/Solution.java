// Day 1214: Reconstruct itinerary using all flights, lexicographically smallest.
// Hierholzer's Eulerian path over sorted multigraph adjacency. Time O(E log E), Space O(E).
import java.util.*;

public class Solution {
    static List<String> findItinerary(String[][] flights, String start) {
        Map<String, PriorityQueue<String>> adj = new HashMap<>();
        for (String[] f : flights) adj.computeIfAbsent(f[0], k -> new PriorityQueue<>()).add(f[1]);
        int total = flights.length;
        LinkedList<String> route = new LinkedList<>();
        Deque<String> st = new ArrayDeque<>();
        st.push(start);
        while (!st.isEmpty()) {
            String u = st.peek();
            PriorityQueue<String> pq = adj.get(u);
            if (pq != null && !pq.isEmpty()) st.push(pq.poll());
            else route.addFirst(st.pop());
        }
        if (route.size() != total + 1) return null;
        return route;
    }

    public static void main(String[] args) {
        String[][] flights = {{"SFO","HKO"},{"YYZ","SFO"},{"YUL","YYZ"},{"HKO","ORD"}};
        List<String> r = findItinerary(flights, "YUL");
        if (r == null) { System.out.println("null"); return; }
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < r.size(); i++) { if (i > 0) sb.append(", "); sb.append("'").append(r.get(i)).append("'"); }
        sb.append("]");
        System.out.println(sb); // ['YUL', 'YYZ', 'SFO', 'HKO', 'ORD']
    }
}
