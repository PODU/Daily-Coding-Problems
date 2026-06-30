// Approach: Single-process simulation of a distributed BFS web crawler. Central frontier (queue) +
// visited set for dedup; N round-robin workers fetch outlinks from a mock graph, store to a mock DB,
// requeue work from a blacklisted worker. Time O(V+E), Space O(V+E).
//
// Real distributed design: a master holds the frontier and shards the "URL-seen" set across nodes via
// consistent hashing or a bloom filter; workers crawl politely (robots.txt, rate limits, rotating IPs)
// to avoid blacklisting; recrawl uses last-modified/ETag to detect and update changed pages.

import java.util.*;

public class Solution {
    public static void main(String[] args) {
        Map<String, List<String>> graph = new HashMap<>();
        graph.put("Main", Arrays.asList("A", "B", "C"));
        graph.put("A", Arrays.asList("B", "D"));
        graph.put("B", Arrays.asList("C"));
        graph.put("C", Arrays.asList("A", "E"));
        graph.put("D", Arrays.asList("E"));
        graph.put("E", Collections.emptyList());

        int numWorkers = 3;
        Deque<String> frontier = new ArrayDeque<>();
        frontier.add("Main");
        Set<String> visited = new HashSet<>(Collections.singletonList("Main"));
        Map<String, List<String>> db = new HashMap<>();
        Set<Integer> blacklisted = new HashSet<>();
        int worker = 0;
        boolean processedAny = false;

        while (!frontier.isEmpty()) {
            String url = frontier.pollFirst();
            int w = worker % numWorkers;
            worker++;

            // Blacklist worker #1 after at least one page processed; requeue its task.
            if (w == 1 && processedAny && !blacklisted.contains(1)) {
                blacklisted.add(1);
                frontier.addLast(url); // in-flight task requeued, no page lost
                continue;
            }

            db.put(url, graph.get(url)); // "fetch" + store to mock DB
            processedAny = true;

            for (String link : graph.get(url)) {
                if (!visited.contains(link)) {
                    visited.add(link);
                    frontier.addLast(link);
                }
            }
        }

        System.out.println("Crawled " + db.size() + " pages");
        List<String> titles = new ArrayList<>(db.keySet());
        Collections.sort(titles);
        System.out.println(String.join(" ", titles));
    }
}
