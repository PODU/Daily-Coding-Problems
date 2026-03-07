// Distributed Wikipedia crawler simulation: master holds a FIFO URL frontier +
// visited set (dedup); workers "fetch" mock pages, parse links, report back; BFS.
// Real design: shard frontier by URL hash (load-balance), distributed visited set
// (bloom filter / consistent hashing), rotating IPs + politeness/backoff to avoid
// blacklisting, recrawl via Last-Modified timestamps. Time/Space: O(V + E).
import java.util.*;

public class Solution {
    public static void main(String[] args) {
        // Mock in-memory "Wikipedia" link graph (adjacency, ordered).
        Map<String, List<String>> graph = new HashMap<>();
        graph.put("Wikipedia", Arrays.asList("Computer_Science", "Mathematics"));
        graph.put("Computer_Science", Arrays.asList("Algorithms", "Mathematics"));
        graph.put("Mathematics", Arrays.asList("Algorithms"));
        graph.put("Algorithms", Collections.emptyList());

        String seed = "Wikipedia";
        Queue<String> frontier = new LinkedList<>();   // master FIFO frontier
        Set<String> seen = new HashSet<>();            // dedup set
        Map<String, List<String>> db = new HashMap<>(); // crawled database

        frontier.add(seed);
        seen.add(seed);
        while (!frontier.isEmpty()) {
            String url = frontier.poll();
            List<String> links = graph.getOrDefault(url, Collections.emptyList()); // worker fetch+parse
            db.put(url, links);
            System.out.println("Crawled: " + url);
            for (String nxt : links) {
                if (!seen.contains(nxt)) { seen.add(nxt); frontier.add(nxt); }
            }
        }
    }
}
