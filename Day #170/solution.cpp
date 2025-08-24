// Word ladder via BFS over one-letter transformations. O(N * L * 26) time, O(N) space.
#include <iostream>
#include <vector>
#include <string>
#include <unordered_set>
#include <unordered_map>
#include <queue>
#include <algorithm>
using namespace std;

vector<string> ladder(const string& start, const string& end, unordered_set<string> dict) {
    if (start == end) return {start};
    queue<string> q;
    q.push(start);
    unordered_map<string, string> parent;
    parent[start] = "";
    while (!q.empty()) {
        string cur = q.front(); q.pop();
        for (int i = 0; i < (int)cur.size(); i++) {
            string nxt = cur;
            for (char c = 'a'; c <= 'z'; c++) {
                if (c == cur[i]) continue;
                nxt[i] = c;
                if (dict.count(nxt) && !parent.count(nxt)) {
                    parent[nxt] = cur;
                    if (nxt == end) {
                        vector<string> path;
                        for (string s = end; !s.empty(); s = parent[s]) path.push_back(s);
                        reverse(path.begin(), path.end());
                        return path;
                    }
                    q.push(nxt);
                }
            }
        }
    }
    return {};
}

void printPath(const vector<string>& path) {
    if (path.empty()) { cout << "null" << endl; return; }
    cout << "[";
    for (size_t i = 0; i < path.size(); i++) { if (i) cout << ", "; cout << "\"" << path[i] << "\""; }
    cout << "]" << endl;
}

int main() {
    printPath(ladder("dog", "cat", {"dot", "dop", "dat", "cat"}));
    printPath(ladder("dog", "cat", {"dot", "tod", "dat", "dar"}));
    return 0;
}
