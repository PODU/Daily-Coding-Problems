// Word ladder: BFS over words, change one letter per step, track predecessors.
// Time: O(N * L * 26), Space: O(N). Returns shortest path or empty (null).
#include <bits/stdc++.h>
using namespace std;

vector<string> ladder(string start, string end, set<string> dict) {
    if (start == end) return {start};
    queue<string> q;
    q.push(start);
    unordered_map<string, string> parent;
    parent[start] = "";
    unordered_set<string> visited{start};

    while (!q.empty()) {
        string cur = q.front(); q.pop();
        for (int i = 0; i < (int)cur.size(); i++) {
            string nxt = cur;
            for (char c = 'a'; c <= 'z'; c++) {
                if (c == cur[i]) continue;
                nxt[i] = c;
                if (dict.count(nxt) && !visited.count(nxt)) {
                    visited.insert(nxt);
                    parent[nxt] = cur;
                    if (nxt == end) {
                        vector<string> path;
                        for (string w = end; !w.empty(); w = parent[w]) path.push_back(w);
                        reverse(path.begin(), path.end());
                        return path;
                    }
                    q.push(nxt);
                }
            }
        }
    }
    return {};  // no path
}

void printPath(const vector<string>& p) {
    if (p.empty()) { cout << "null\n"; return; }
    cout << "[";
    for (size_t i = 0; i < p.size(); i++) {
        cout << "\"" << p[i] << "\"";
        if (i + 1 < p.size()) cout << ", ";
    }
    cout << "]\n";
}

int main() {
    printPath(ladder("dog", "cat", {"dot", "dop", "dat", "cat"}));  // ["dog", "dot", "dat", "cat"]
    printPath(ladder("dog", "cat", {"dot", "tod", "dat", "dar"}));  // null
    return 0;
}
