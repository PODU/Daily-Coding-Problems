// Day 1031: Snakes & Ladders min turns. BFS over squares 1..100, each turn rolls 1..6,
// applying snake/ladder mapping when a roll lands on a key. Time O(100*6), Space O(100).
#include <bits/stdc++.h>
using namespace std;

int minTurns() {
    unordered_map<int, int> jumps = {
        {16, 6}, {48, 26}, {49, 11}, {56, 53}, {62, 19}, {64, 60}, {87, 24}, {93, 73}, {95, 75}, {98, 78},
        {1, 38}, {4, 14}, {9, 31}, {21, 42}, {28, 84}, {36, 44}, {51, 67}, {71, 91}, {80, 100}};
    vector<int> dist(101, -1);
    queue<int> q;
    dist[1] = 0;            // start at square 1 (mapping applied only on landing after a roll)
    q.push(1);
    while (!q.empty()) {
        int s = q.front(); q.pop();
        if (s == 100) return dist[s];
        for (int d = 1; d <= 6; ++d) {
            int nx = s + d;
            if (nx > 100) continue;
            if (jumps.count(nx)) nx = jumps[nx];
            if (dist[nx] == -1) { dist[nx] = dist[s] + 1; q.push(nx); }
        }
    }
    return -1;
}

int main() { cout << minTurns() << "\n"; }
