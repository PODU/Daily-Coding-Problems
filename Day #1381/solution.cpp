// Conway's Game of Life on an infinite board via a hash-set of live coords.
// Each step counts neighbours only around live cells.
// Time: O(L) per step, Space: O(L).
#include <bits/stdc++.h>
using namespace std;

struct PH { size_t operator()(const pair<int,int>& p) const {
    return hash<long long>()(((long long)p.first << 32) ^ (unsigned)p.second);
}};

using Cell = pair<int,int>;
using Board = unordered_set<Cell, PH>;

Board step(const Board& live) {
    unordered_map<Cell, int, PH> cnt;
    for (auto& c : live)
        for (int dx = -1; dx <= 1; ++dx)
            for (int dy = -1; dy <= 1; ++dy)
                if (dx || dy) cnt[{c.first + dx, c.second + dy}]++;
    Board nb;
    for (auto& kv : cnt) {
        int c = kv.second;
        if (c == 3 || (c == 2 && live.count(kv.first))) nb.insert(kv.first);
    }
    return nb;
}

void render(const Board& live) {
    if (live.empty()) { cout << "(empty)\n"; return; }
    int minx = INT_MAX, maxx = INT_MIN, miny = INT_MAX, maxy = INT_MIN;
    for (auto& c : live) {
        minx = min(minx, c.first);  maxx = max(maxx, c.first);
        miny = min(miny, c.second); maxy = max(maxy, c.second);
    }
    for (int x = minx; x <= maxx; ++x) {
        string row;
        for (int y = miny; y <= maxy; ++y)
            row += live.count({x, y}) ? '*' : '.';
        cout << row << "\n";
    }
}

int main() {
    Board live = {{1,0},{1,1},{1,2}}; // blinker
    int steps = 2;
    for (int i = 0; i <= steps; ++i) {
        cout << "Step " << i << ":\n";
        render(live);
        cout << "\n";
        live = step(live);
    }
    return 0;
}
