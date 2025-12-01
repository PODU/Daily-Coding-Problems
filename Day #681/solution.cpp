// BFS over squares 1..100; from s try rolls 1..6, apply snake/ladder mapping. Time O(N*6), Space O(N).
#include <bits/stdc++.h>
using namespace std;

int minTurns() {
    map<int, int> jump = {
        {16, 6}, {48, 26}, {49, 11}, {56, 53}, {62, 19}, {64, 60},
        {87, 24}, {93, 73}, {95, 75}, {98, 78},                      // snakes
        {1, 38}, {4, 14}, {9, 31}, {21, 42}, {28, 84}, {36, 44},
        {51, 67}, {71, 91}, {80, 100}                                // ladders
    };
    vector<int> dist(101, -1);
    queue<int> q;
    q.push(1); dist[1] = 0;                 // start placed on 1; do NOT apply 1->38 here
    while (!q.empty()) {
        int s = q.front(); q.pop();
        if (s == 100) return dist[s];
        for (int r = 1; r <= 6; ++r) {
            int nxt = s + r;
            if (nxt > 100) continue;
            if (jump.count(nxt)) nxt = jump[nxt];
            if (dist[nxt] == -1) { dist[nxt] = dist[s] + 1; q.push(nxt); }
        }
    }
    return dist[100];
}

int main() {
    cout << "Minimum turns: " << minTurns() << "\n";
    return 0;
}
