// Day 439: Reconstruct itinerary using all flights, lexicographically smallest.
// Hierholzer's Eulerian-path algorithm with min-heap adjacency. O(E log E).
import java.util.*;

public class Solution {
    static List<String> findItinerary(String[][] flights, String start) {
        Map<String, PriorityQueue<String>> graph = new HashMap<>();
        for (String[] f : flights)
            graph.computeIfAbsent(f[0], k -> new PriorityQueue<>()).add(f[1]);

        LinkedList<String> route = new LinkedList<>();
        Deque<String> stack = new ArrayDeque<>();
        stack.push(start);
        while (!stack.isEmpty()) {
            String u = stack.peek();
            PriorityQueue<String> pq = graph.get(u);
            if (pq != null && !pq.isEmpty()) {
                stack.push(pq.poll());
            } else {
                route.addFirst(stack.pop());
            }
        }
        if (route.size() != flights.length + 1) return null; // no valid itinerary
        return route;
    }

    static void printRoute(List<String> res) {
        if (res == null) { System.out.println("null"); return; }
        StringBuilder sb = new StringBuilder("[");
        for (int i = 0; i < res.size(); i++) {
            sb.append("'").append(res.get(i)).append("'");
            if (i + 1 < res.size()) sb.append(", ");
        }
        sb.append("]");
        System.out.println(sb);
    }

    public static void main(String[] args) {
        String[][] flights = {{"SFO","HKO"},{"YYZ","SFO"},{"YUL","YYZ"},{"HKO","ORD"}};
        printRoute(findItinerary(flights, "YUL"));
        // ['YUL', 'YYZ', 'SFO', 'HKO', 'ORD']
        printRoute(findItinerary(new String[][]{{"SFO","COM"},{"COM","YYZ"}}, "COM")); // null
        printRoute(findItinerary(new String[][]{{"A","B"},{"A","C"},{"B","C"},{"C","A"}}, "A"));
        // ['A', 'B', 'C', 'A', 'C']
    }
}
