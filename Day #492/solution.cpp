// Day 492: Graph m-colorability via backtracking.
// Assign each vertex a color 1..k, ensuring no adjacent pair (adjacency matrix) matches.
// Time: O(k^V) worst case, Space: O(V) for the color assignment + recursion stack.
#include <bits/stdc++.h>
using namespace std;

bool isSafe(int v, const vector<vector<int>>& graph, const vector<int>& colors, int c) {
    for (int u = 0; u < (int)graph.size(); ++u)
        if (graph[v][u] && colors[u] == c) return false;
    return true;
}

bool solve(int v, const vector<vector<int>>& graph, int k, vector<int>& colors) {
    if (v == (int)graph.size()) return true;
    for (int c = 1; c <= k; ++c) {
        if (isSafe(v, graph, colors, c)) {
            colors[v] = c;
            if (solve(v + 1, graph, k, colors)) return true;
            colors[v] = 0;
        }
    }
    return false;
}

bool canColor(const vector<vector<int>>& graph, int k) {
    vector<int> colors(graph.size(), 0);
    return solve(0, graph, k, colors);
}

int main() {
    // Triangle K3: every pair adjacent.
    vector<vector<int>> graph = {
        {0, 1, 1},
        {1, 0, 1},
        {1, 1, 0}};
    cout << "k=2 colorable: " << (canColor(graph, 2) ? "true" : "false") << "\n";
    cout << "k=3 colorable: " << (canColor(graph, 3) ? "true" : "false") << "\n";
    return 0;
}
