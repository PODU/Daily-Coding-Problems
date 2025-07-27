// Game of Life on infinite board: track live cells in a set, count neighbors via a
// map over live cells + neighbors each step. O(live*9) per step. Print cropped board.
#include <iostream>
#include <set>
#include <map>
using namespace std;

typedef pair<int,int> Cell; // (x, y)

set<Cell> step(const set<Cell>& live) {
    map<Cell,int> counts;
    for (auto& c : live)
        for (int dx = -1; dx <= 1; dx++)
            for (int dy = -1; dy <= 1; dy++)
                if (dx || dy) counts[{c.first+dx, c.second+dy}]++;
    set<Cell> next;
    for (auto& kv : counts) {
        int n = kv.second;
        bool alive = live.count(kv.first);
        if (n == 3 || (alive && n == 2)) next.insert(kv.first);
    }
    return next;
}

void printBoard(const set<Cell>& live) {
    int minx = 1e9, maxx = -1e9, miny = 1e9, maxy = -1e9;
    for (auto& c : live) {
        minx = min(minx, c.first); maxx = max(maxx, c.first);
        miny = min(miny, c.second); maxy = max(maxy, c.second);
    }
    for (int y = miny; y <= maxy; y++) {
        string row;
        for (int x = minx; x <= maxx; x++)
            row += live.count({x, y}) ? '*' : '.';
        cout << row << "\n";
    }
}

int main() {
    set<Cell> live = {{0,1},{1,1},{2,1}};
    int steps = 2;
    for (int s = 0; s <= steps; s++) {
        cout << "Step " << s << ":\n";
        printBoard(live);
        if (s < steps) live = step(live);
    }
    return 0;
}
