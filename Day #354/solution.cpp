// Distributed crawler design (simulation): Coordinator holds a FIFO URL frontier + central visited set.
// Workers pull a URL, "download" (map lookup), record it, push unvisited links. BFS over a mock graph.
// Architecture: reach pages = BFS from seeds following links; visited tracking = central set/bloom filter on
// coordinator; blacklisting = rotate client IPs/user-agents, rate-limit + exponential backoff; updates =
// recrawl by last-modified via a priority queue. Time O(V+E), Space O(V) for visited + frontier.
#include <iostream>
#include <queue>
#include <unordered_set>
#include <unordered_map>
#include <vector>
#include <string>
using namespace std;

int main() {
    unordered_map<string, vector<string>> wiki = {
        {"/Main",   {"/Apple", "/Banana"}},
        {"/Apple",  {"/Banana", "/Fruit"}},
        {"/Banana", {"/Fruit"}},
        {"/Fruit",  {"/Main"}},
    };

    vector<string> seeds = {"/Main"};
    queue<string> frontier;
    unordered_set<string> visited;
    vector<string> order;

    for (const auto& s : seeds) {
        if (!visited.count(s)) { visited.insert(s); frontier.push(s); }
    }

    while (!frontier.empty()) {
        string url = frontier.front(); frontier.pop();
        order.push_back(url); // "download" + record into results DB
        auto it = wiki.find(url);
        if (it != wiki.end()) {
            for (const auto& link : it->second) {
                if (!visited.count(link)) { visited.insert(link); frontier.push(link); }
            }
        }
    }

    cout << "Crawled " << order.size() << " pages:" << endl;
    for (const auto& p : order) cout << p << endl;
    return 0;
}
