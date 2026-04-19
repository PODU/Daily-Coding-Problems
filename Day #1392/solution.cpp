// Circular lock min moves via BFS over 1000 states; each state has 6 neighbors (3 wheels x +/-1 mod 10). O(states) time/space.
#include <bits/stdc++.h>
using namespace std;

int minMoves(const string& target, const set<string>& deadends) {
    if (deadends.count("000") || deadends.count(target)) return -1;
    if (target == "000") return 0;
    set<string> visited{"000"};
    queue<pair<string,int>> q;
    q.push({"000", 0});
    while (!q.empty()) {
        auto [cur, d] = q.front(); q.pop();
        for (int i = 0; i < 3; i++) {
            for (int delta : {1, 9}) {
                string nx = cur;
                nx[i] = '0' + ((cur[i] - '0' + delta) % 10);
                if (visited.count(nx) || deadends.count(nx)) continue;
                if (nx == target) return d + 1;
                visited.insert(nx);
                q.push({nx, d + 1});
            }
        }
    }
    return -1;
}

int main() {
    set<string> deadends = {"100", "020", "001"};
    cout << minMoves("333", deadends) << "\n";
    return 0;
}
