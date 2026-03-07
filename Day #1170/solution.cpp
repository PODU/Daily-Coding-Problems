// Distributed Wikipedia crawler simulation: master holds a FIFO URL frontier +
// visited set (dedup); workers "fetch" mock pages, parse links, report back; BFS.
// Real design: shard frontier by URL hash (load-balance), distributed visited set
// (bloom filter / consistent hashing), rotating IPs + politeness/backoff to avoid
// blacklisting, recrawl via Last-Modified timestamps. Time/Space: O(V + E).
#include <bits/stdc++.h>
using namespace std;

int main() {
    // Mock in-memory "Wikipedia" link graph (adjacency, ordered).
    unordered_map<string, vector<string>> graph = {
        {"Wikipedia", {"Computer_Science", "Mathematics"}},
        {"Computer_Science", {"Algorithms", "Mathematics"}},
        {"Mathematics", {"Algorithms"}},
        {"Algorithms", {}},
    };

    auto fetch = [&](const string& url) { return graph[url]; }; // worker fetch+parse

    string seed = "Wikipedia";
    queue<string> frontier;        // master FIFO frontier
    unordered_set<string> seen;    // visited/enqueued dedup set
    unordered_map<string, vector<string>> db; // crawled "database"

    frontier.push(seed);
    seen.insert(seed);
    while (!frontier.empty()) {
        string url = frontier.front(); frontier.pop();
        vector<string> links = fetch(url);   // worker reports content + links
        db[url] = links;
        cout << "Crawled: " << url << "\n";
        for (const string& nxt : links) {
            if (!seen.count(nxt)) { seen.insert(nxt); frontier.push(nxt); }
        }
    }
    return 0;
}
