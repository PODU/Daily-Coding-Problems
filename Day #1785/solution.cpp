// Build origin->sorted destinations; DFS backtracking trying lexicographically
// smallest dest first; first path using all flights is the answer.
// Time O(E!) worst case, Space O(E).
#include <iostream>
#include <algorithm>
#include <map>
#include <vector>
#include <string>
using namespace std;

map<string, vector<pair<string,bool>>> graph; // dest, used
int total;

bool dfs(const string& node, vector<string>& path) {
    if ((int)path.size() == total + 1) return true;
    for (auto& e : graph[node]) {
        if (e.second) continue;
        e.second = true;
        path.push_back(e.first);
        if (dfs(e.first, path)) return true;
        path.pop_back();
        e.second = false;
    }
    return false;
}

vector<string> findItinerary(vector<pair<string,string>> flights, string start) {
    graph.clear();
    total = flights.size();
    for (auto& f : flights) graph[f.first].push_back({f.second, false});
    for (auto& kv : graph) sort(kv.second.begin(), kv.second.end());
    vector<string> path = {start};
    if (dfs(start, path)) return path;
    return {};
}

void printResult(const vector<string>& r) {
    if (r.empty()) { cout << "null\n"; return; }
    cout << "[";
    for (size_t i = 0; i < r.size(); i++) {
        cout << "'" << r[i] << "'";
        if (i + 1 < r.size()) cout << ", ";
    }
    cout << "]\n";
}

int main() {
    printResult(findItinerary({{"SFO","HKO"},{"YYZ","SFO"},{"YUL","YYZ"},{"HKO","ORD"}}, "YUL"));
    printResult(findItinerary({{"SFO","COM"},{"COM","YYZ"}}, "COM"));
    printResult(findItinerary({{"A","B"},{"A","C"},{"B","C"},{"C","A"}}, "A"));
    return 0;
}
