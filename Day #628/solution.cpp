// Combination lock: BFS over 1000 states from "000" to target, avoiding deadends.
// Each of 3 wheels has 2 neighbors (+1/-1 mod 10) => 6 neighbors. Time/space O(1000).
#include <bits/stdc++.h>
using namespace std;

int openLock(const vector<string>& deadends, const string& target, const string& start = "000") {
    unordered_set<string> dead(deadends.begin(), deadends.end());
    if (dead.count(start)) return -1;
    if (start == target) return 0;
    unordered_set<string> visited{start};
    queue<string> q;
    q.push(start);
    int steps = 0;
    while (!q.empty()) {
        int sz = (int)q.size();
        steps++;
        for (int i = 0; i < sz; i++) {
            string cur = q.front(); q.pop();
            for (int w = 0; w < 3; w++) {
                for (int d : {1, -1}) {
                    string nxt = cur;
                    nxt[w] = char('0' + ((cur[w] - '0' + d + 10) % 10));
                    if (dead.count(nxt) || visited.count(nxt)) continue;
                    if (nxt == target) return steps;
                    visited.insert(nxt);
                    q.push(nxt);
                }
            }
        }
    }
    return -1;
}

int main() {
    vector<string> dead1 = {"010", "021"};
    int r1 = openLock(dead1, "020");
    cout << "Min moves to open lock (target 020): " << r1 << "\n";

    vector<string> dead2 = {"001","010","100","009","090","900"};
    int r2 = openLock(dead2, "111");
    cout << "Impossible case (target 111): " << (r2 == -1 ? "None" : to_string(r2)) << "\n";
    return 0;
}
