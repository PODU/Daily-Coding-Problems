// Day 901: Distributed Wikipedia crawler (concrete simulation).
// Approach: BFS over a page link graph with a shared visited set (dedup) and a
// frontier queue. Production: distributed frontier, sharded/bloom visited store,
// rotating rate-limited clients to avoid bans, RecentChanges-driven re-crawl.
// Time: O(V+E), Space: O(V).
#include <bits/stdc++.h>
using namespace std;

class CrawlerSystem {
    unordered_map<string, vector<string>> linkGraph;
    unordered_set<string> visited;          // central dedup store
public:
    map<string, string> db;                 // page -> stored content
    CrawlerSystem(unordered_map<string, vector<string>> g) : linkGraph(move(g)) {}

    void crawl(const vector<string>& seeds) {
        queue<string> frontier;             // distributed work queue
        for (auto& s : seeds) { visited.insert(s); frontier.push(s); }
        while (!frontier.empty()) {
            string page = frontier.front(); frontier.pop();
            db[page] = "content of " + page;
            for (auto& nxt : linkGraph[page]) {
                if (!visited.count(nxt)) {  // dedup before enqueue
                    visited.insert(nxt);
                    frontier.push(nxt);
                }
            }
        }
    }
};

int main() {
    unordered_map<string, vector<string>> graph = {
        {"Main_Page", {"Python", "Wikipedia"}},
        {"Python", {"Programming", "Wikipedia"}},
        {"Wikipedia", {"Python"}},
        {"Programming", {}},
    };
    CrawlerSystem sys(graph);
    sys.crawl({"Main_Page"});
    cout << "Pages crawled: " << sys.db.size() << "\n";
    cout << "Visited:";
    for (auto& kv : sys.db) cout << " " << kv.first;
    cout << "\n";
    return 0;
}
