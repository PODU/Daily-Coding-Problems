// Snakes and Ladders: BFS over squares 1..100, each die roll (1..6) is one edge; apply jumps.
// Time: O(100 * 6), Space: O(100).
#include <bits/stdc++.h>
using namespace std;

int minTurns() {
    map<int, int> jump = {
        {16, 6}, {48, 26}, {49, 11}, {56, 53}, {62, 19}, {64, 60}, {87, 24}, {93, 73}, {95, 75}, {98, 78},
        {1, 38}, {4, 14}, {9, 31}, {21, 42}, {28, 84}, {36, 44}, {51, 67}, {71, 91}, {80, 100}};
    auto apply = [&](int p) { return jump.count(p) ? jump[p] : p; };
    vector<int> dist(101, -1);
    int start = apply(1);
    queue<int> q;
    q.push(start);
    dist[start] = 0;
    while (!q.empty()) {
        int p = q.front(); q.pop();
        if (p == 100) return dist[p];
        for (int d = 1; d <= 6; d++) {
            int np = p + d;
            if (np > 100) continue;
            np = apply(np);
            if (dist[np] == -1) { dist[np] = dist[p] + 1; q.push(np); }
        }
    }
    return dist[100];
}

int main() {
    cout << "Minimum turns: " << minTurns() << "\n";
}
