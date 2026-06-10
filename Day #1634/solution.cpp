// Word ladder: BFS over dictionary words (one-letter changes), tracking parents to rebuild shortest path. O(N*L*N) time.
#include <bits/stdc++.h>
using namespace std;

bool differsByOne(const string& a, const string& b) {
    if (a.size() != b.size()) return false;
    int diff = 0;
    for (size_t i = 0; i < a.size(); i++)
        if (a[i] != b[i] && ++diff > 1) return false;
    return diff == 1;
}

// returns empty vector if no path
vector<string> ladder(const string& start, const string& end, const vector<string>& dict) {
    set<string> visited{start};
    queue<string> q;
    q.push(start);
    unordered_map<string, string> parent;
    while (!q.empty()) {
        string cur = q.front(); q.pop();
        if (cur == end) {
            vector<string> path;
            string at = cur;
            while (true) {
                path.push_back(at);
                if (at == start) break;
                at = parent[at];
            }
            reverse(path.begin(), path.end());
            return path;
        }
        for (const string& w : dict) {
            if (!visited.count(w) && differsByOne(cur, w)) {
                visited.insert(w);
                parent[w] = cur;
                q.push(w);
            }
        }
    }
    return {};
}

void printPath(const vector<string>& path) {
    if (path.empty()) { cout << "null\n"; return; }
    cout << "[";
    for (size_t i = 0; i < path.size(); i++) {
        if (i) cout << ", ";
        cout << "\"" << path[i] << "\"";
    }
    cout << "]\n";
}

int main() {
    printPath(ladder("dog", "cat", {"dot", "dop", "dat", "cat"}));
    printPath(ladder("dog", "cat", {"tod", "dat", "dar", "dot"}));
    return 0;
}
