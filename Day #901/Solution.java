// Day 901: Distributed Wikipedia crawler (concrete simulation).
// Approach: BFS over a page link graph with a shared visited set (dedup) and a
// frontier queue. Production: distributed frontier queue, sharded/bloom visited
// store, rotating rate-limited clients to avoid bans, RecentChanges-driven
// re-crawl. Time: O(V+E), Space: O(V).
import java.util.*;

public class Solution {
    static class CrawlerSystem {
        Map<String, List<String>> linkGraph;
        Set<String> visited = new HashSet<>();          // central dedup store
        Map<String, String> db = new TreeMap<>();       // page -> content

        CrawlerSystem(Map<String, List<String>> g) { this.linkGraph = g; }

        void crawl(List<String> seeds) {
            Deque<String> frontier = new ArrayDeque<>(); // distributed queue
            for (String s : seeds) { visited.add(s); frontier.add(s); }
            while (!frontier.isEmpty()) {
                String page = frontier.poll();
                db.put(page, "content of " + page);
                for (String nxt : linkGraph.getOrDefault(page, List.of())) {
                    if (!visited.contains(nxt)) {        // dedup before enqueue
                        visited.add(nxt);
                        frontier.add(nxt);
                    }
                }
            }
        }
    }

    public static void main(String[] args) {
        Map<String, List<String>> graph = new HashMap<>();
        graph.put("Main_Page", List.of("Python", "Wikipedia"));
        graph.put("Python", List.of("Programming", "Wikipedia"));
        graph.put("Wikipedia", List.of("Python"));
        graph.put("Programming", List.of());
        CrawlerSystem sys = new CrawlerSystem(graph);
        sys.crawl(List.of("Main_Page"));
        System.out.println("Pages crawled: " + sys.db.size());
        System.out.println("Visited: " + sys.db.keySet());
    }
}
