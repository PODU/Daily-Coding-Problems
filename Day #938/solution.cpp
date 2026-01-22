// Day 938: Min moves on a 3-wheel lock from 000 to target, avoiding dead ends. BFS over
// 1000 states, each with 6 neighbors. Time O(1000*6), Space O(1000). Returns -1 (None) if blocked.
#include <bits/stdc++.h>
using namespace std;

int minMoves(const string& target, const set<string>& dead) {
    string start = "000";
    if (dead.count(start) || dead.count(target)) return -1;
    if (start == target) return 0;
    set<string> visited{start};
    queue<pair<string,int>> q;
    q.push({start, 0});
    while (!q.empty()) {
        string cur = q.front().first; int d = q.front().second; q.pop();
        for (int i = 0; i < 3; ++i) {
            for (int dir : {1, 9}) { // +1 or -1 (mod 10)
                string nx = cur;
                nx[i] = '0' + ((cur[i] - '0') + dir) % 10;
                if (dead.count(nx) || visited.count(nx)) continue;
                if (nx == target) return d + 1;
                visited.insert(nx);
                q.push({nx, d + 1});
            }
        }
    }
    return -1;
}

int main() {
    cout << minMoves("123", {}) << "\n"; // 6
    set<string> dead = {"100","900","010","090","001","009"}; // seal off 000
    int r = minMoves("111", dead);
    cout << (r == -1 ? "None" : to_string(r)) << "\n"; // None
    return 0;
}
