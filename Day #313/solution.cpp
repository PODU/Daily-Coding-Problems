// Day 313: Min moves on a 3-wheel lock from 000 to target, avoiding dead ends.
// BFS over <=1000 states. O(1000) time. Prints -1 to represent None.
#include <bits/stdc++.h>
using namespace std;
int openLock(string target, vector<string> deadends) {
    set<string> dead(deadends.begin(), deadends.end());
    string start = "000";
    if (dead.count(start)) return -1;
    if (start == target) return 0;
    queue<string> q; q.push(start);
    map<string, int> dist; dist[start] = 0;
    while (!q.empty()) {
        string cur = q.front(); q.pop();
        for (int i = 0; i < 3; i++) for (int d = -1; d <= 1; d += 2) {
            string nx = cur;
            nx[i] = '0' + ((cur[i] - '0' + d + 10) % 10);
            if (dead.count(nx) || dist.count(nx)) continue;
            dist[nx] = dist[cur] + 1;
            if (nx == target) return dist[nx];
            q.push(nx);
        }
    }
    return -1; // None
}
int main() {
    cout << openLock("123", {}) << "\n"; // 6
}
