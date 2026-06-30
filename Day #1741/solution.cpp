// Approach: Single-process simulation of a distributed BFS web crawler. Central frontier (queue) +
// visited set for dedup; N round-robin workers fetch outlinks from a mock graph, store to a mock DB,
// requeue work from a blacklisted worker. Time O(V+E), Space O(V+E).
//
// Real distributed design: a master holds the frontier and shards the "URL-seen" set across nodes via
// consistent hashing or a bloom filter; workers crawl politely (robots.txt, rate limits, rotating IPs)
// to avoid blacklisting; recrawl uses last-modified/ETag to detect and update changed pages.

#include <bits/stdc++.h>
using namespace std;

int main() {
    map<string, vector<string>> graph = {
        {"Main", {"A", "B", "C"}},
        {"A", {"B", "D"}},
        {"B", {"C"}},
        {"C", {"A", "E"}},
        {"D", {"E"}},
        {"E", {}},
    };

    int numWorkers = 3;
    deque<string> frontier{"Main"};
    set<string> visited{"Main"};
    map<string, vector<string>> db;
    set<int> blacklisted;
    int worker = 0;
    bool processedAny = false;

    while (!frontier.empty()) {
        string url = frontier.front();
        frontier.pop_front();
        int w = worker % numWorkers;
        worker++;

        // Blacklist worker #1 after at least one page processed; requeue its task.
        if (w == 1 && processedAny && !blacklisted.count(1)) {
            blacklisted.insert(1);
            frontier.push_back(url); // in-flight task requeued, no page lost
            continue;
        }

        db[url] = graph[url]; // "fetch" + store to mock DB
        processedAny = true;

        for (const string& link : graph[url]) {
            if (!visited.count(link)) {
                visited.insert(link);
                frontier.push_back(link);
            }
        }
    }

    cout << "Crawled " << db.size() << " pages\n";
    vector<string> titles;
    for (auto& kv : db) titles.push_back(kv.first);
    sort(titles.begin(), titles.end());
    for (size_t i = 0; i < titles.size(); i++) {
        cout << titles[i];
        if (i + 1 < titles.size()) cout << " ";
    }
    cout << "\n";
    return 0;
}
