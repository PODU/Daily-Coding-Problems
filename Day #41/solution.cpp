// Reconstruct Itinerary: backtracking DFS over lexicographically sorted edges.
// First complete itinerary using all edges (tried in lex order) is the answer.
// Time: exponential worst case; Space: O(E).
#include <bits/stdc++.h>
using namespace std;

static map<string, vector<string>> adj;
static map<string, vector<char>> used;
static int totalEdges;
static vector<string> path;

bool dfs(const string& node) {
    if ((int)path.size() == totalEdges + 1) return true;
    auto it = adj.find(node);
    if (it == adj.end()) return false;
    auto& dests = it->second;
    auto& u = used[node];
    for (size_t i = 0; i < dests.size(); i++) {
        if (u[i]) continue;
        u[i] = 1;
        path.push_back(dests[i]);
        if (dfs(dests[i])) return true;
        path.pop_back();
        u[i] = 0;
    }
    return false;
}

vector<string> reconstruct(vector<pair<string,string>> flights, const string& start) {
    adj.clear(); used.clear();
    for (auto& f : flights) adj[f.first].push_back(f.second);
    for (auto& kv : adj) {
        sort(kv.second.begin(), kv.second.end());
        used[kv.first] = vector<char>(kv.second.size(), 0);
    }
    totalEdges = (int)flights.size();
    path.clear();
    path.push_back(start);
    if (dfs(start)) return path;
    return {};
}

string fmt(const vector<string>& v) {
    if (v.empty()) return "null";
    string s = "[";
    for (size_t i = 0; i < v.size(); i++) {
        s += "'" + v[i] + "'";
        if (i + 1 < v.size()) s += ", ";
    }
    return s + "]";
}

int main() {
    cout << fmt(reconstruct({{"SFO","HKO"},{"YYZ","SFO"},{"YUL","YYZ"},{"HKO","ORD"}}, "YUL")) << "\n";
    cout << fmt(reconstruct({{"SFO","COM"},{"COM","YYZ"}}, "COM")) << "\n";
    cout << fmt(reconstruct({{"A","B"},{"A","C"},{"B","C"},{"C","A"}}, "A")) << "\n";
    return 0;
}
