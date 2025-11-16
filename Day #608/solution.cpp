// Word Ladder: BFS over words, mutating one letter at a time; track parents to rebuild path.
// Time: O(N * L * 26) where N=dict size, L=word length. Space: O(N * L).
#include <bits/stdc++.h>
using namespace std;

vector<string> wordLadder(const string& start, const string& end,
                          unordered_set<string> dict) {
    if (dict.find(end) == dict.end()) return {};
    queue<string> q;
    q.push(start);
    unordered_map<string, string> parent;
    parent[start] = "";
    while (!q.empty()) {
        string cur = q.front(); q.pop();
        if (cur == end) {
            vector<string> path;
            for (string w = end; !w.empty() || w == start;) {
                path.push_back(w);
                if (w == start) break;
                w = parent[w];
            }
            reverse(path.begin(), path.end());
            return path;
        }
        for (size_t i = 0; i < cur.size(); ++i) {
            string nxt = cur;
            for (char c = 'a'; c <= 'z'; ++c) {
                if (c == cur[i]) continue;
                nxt[i] = c;
                if (dict.find(nxt) != dict.end() && parent.find(nxt) == parent.end()) {
                    parent[nxt] = cur;
                    q.push(nxt);
                }
            }
        }
    }
    return {};
}

void printPath(const vector<string>& path) {
    if (path.empty()) { cout << "null\n"; return; }
    cout << "[";
    for (size_t i = 0; i < path.size(); ++i) {
        cout << "\"" << path[i] << "\"";
        if (i + 1 < path.size()) cout << ", ";
    }
    cout << "]\n";
}

int main() {
    printPath(wordLadder("dog", "cat", {"dot", "dop", "dat", "cat"}));
    printPath(wordLadder("dog", "cat", {"dot", "tod", "dat", "dar"}));
    return 0;
}
