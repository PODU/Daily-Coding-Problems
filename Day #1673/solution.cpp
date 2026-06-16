// Day 1673: Min wheel rotations from "000" to target avoiding dead ends.
// BFS over <=1000 states, each with 6 neighbors. Time O(1000), Space O(1000).
#include <bits/stdc++.h>
using namespace std;

int openLock(const string& target, const vector<string>& deadends) {
    set<string> dead(deadends.begin(), deadends.end());
    string start = "000";
    if (dead.count(start) || dead.count(target)) return -1; // None
    if (start == target) return 0;
    queue<string> q; q.push(start);
    map<string,int> dist; dist[start] = 0;
    while (!q.empty()) {
        string cur = q.front(); q.pop();
        for (int i = 0; i < 3; ++i) {
            for (int dir : {-1, 1}) {
                string nxt = cur;
                nxt[i] = '0' + ((cur[i] - '0' + dir + 10) % 10);
                if (dead.count(nxt) || dist.count(nxt)) continue;
                dist[nxt] = dist[cur] + 1;
                if (nxt == target) return dist[nxt];
                q.push(nxt);
            }
        }
    }
    return -1; // None
}

int main() {
    int r = openLock("345", {"333"}); // 12
    if (r < 0) cout << "None\n"; else cout << r << "\n";
    return 0;
}
