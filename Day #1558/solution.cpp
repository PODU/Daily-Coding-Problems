// Count connected components (friend groups) in an undirected graph via DFS.
// Time O(V+E), Space O(V).
#include <iostream>
#include <vector>
#include <map>
#include <set>
using namespace std;

int main() {
    map<int, vector<int>> adj = {
        {0, {1, 2}},
        {1, {0, 5}},
        {2, {0}},
        {3, {6}},
        {4, {}},
        {5, {1}},
        {6, {3}}
    };

    set<int> visited;
    int groups = 0;
    for (auto& kv : adj) {
        int start = kv.first;
        if (visited.count(start)) continue;
        groups++;
        vector<int> stack = {start};
        visited.insert(start);
        while (!stack.empty()) {
            int u = stack.back();
            stack.pop_back();
            for (int v : adj[u]) {
                if (!visited.count(v)) {
                    visited.insert(v);
                    stack.push_back(v);
                }
            }
        }
    }
    cout << groups << '\n';
    return 0;
}
