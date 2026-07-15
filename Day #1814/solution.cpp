// Snakes & Ladders minimum turns: BFS over board squares (unweighted shortest path).
// Each square has up to 6 die-roll edges; snakes/ladders redirect the landing square.
// Time: O(100*6). Space: O(100).
#include <bits/stdc++.h>
using namespace std;

int minTurns(map<int,int>& jump, int size = 100) {
    vector<int> dist(size + 1, -1);
    queue<int> q;
    dist[1] = 0; q.push(1); // begin on square 1; jumps trigger only on rolled squares
    while (!q.empty()) {
        int sq = q.front(); q.pop();
        if (sq == size) return dist[sq];
        for (int d = 1; d <= 6; d++) {
            int nxt = sq + d;
            if (nxt > size) continue;
            if (jump.count(nxt)) nxt = jump[nxt];
            if (dist[nxt] == -1) { dist[nxt] = dist[sq] + 1; q.push(nxt); }
        }
    }
    return dist[size];
}

int main() {
    map<int,int> jump = {
        {16,6},{48,26},{49,11},{56,53},{62,19},{64,60},{87,24},{93,73},{95,75},{98,78},
        {1,38},{4,14},{9,31},{21,42},{28,84},{36,44},{51,67},{71,91},{80,100}
    };
    cout << minTurns(jump) << "\n"; // 7
    return 0;
}
