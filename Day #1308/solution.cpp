// Minimum cost to connect all houses to plant = Minimum Spanning Tree weight.
// Prim's algorithm with a min-heap. Time O(E log V), Space O(V + E).
#include <bits/stdc++.h>
using namespace std;

int minCost(unordered_map<string, unordered_map<string,int>>& g) {
    if (g.empty()) return 0;
    unordered_set<string> visited;
    // {weight, node}
    priority_queue<pair<int,string>, vector<pair<int,string>>, greater<>> pq;
    string start = g.begin()->first;
    pq.push({0, start});
    int total = 0;
    while (!pq.empty()) {
        int w = pq.top().first;
        string u = pq.top().second;
        pq.pop();
        if (visited.count(u)) continue;
        visited.insert(u);
        total += w;
        for (unordered_map<string,int>::iterator it = g[u].begin(); it != g[u].end(); ++it)
            if (!visited.count(it->first)) pq.push(make_pair(it->second, it->first));
    }
    return total;
}

int main() {
    // undirected graph (edges added both ways)
    unordered_map<string, unordered_map<string,int>> g;
    auto addEdge = [&](const string& a, const string& b, int c){ g[a][b]=c; g[b][a]=c; };
    addEdge("plant","A",1);
    addEdge("plant","B",5);
    addEdge("plant","C",20);
    addEdge("A","C",15);
    addEdge("B","C",10);
    cout << minCost(g) << "\n"; // 16
    return 0;
}
