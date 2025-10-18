// Day 455: Conway's Game of Life on an infinite board.
// Hash set of live cells; tally neighbours each tick. Time O(live) per step.
#include <iostream>
#include <set>
#include <map>
#include <climits>
using namespace std;

typedef pair<int, int> Cell;

set<Cell> step(const set<Cell>& live) {
    map<Cell, int> cnt;
    for (auto& c : live)
        for (int dr = -1; dr <= 1; dr++)
            for (int dc = -1; dc <= 1; dc++)
                if (dr || dc) cnt[{c.first + dr, c.second + dc}]++;
    set<Cell> next;
    for (auto& kv : cnt) {
        int n = kv.second;
        bool alive = live.count(kv.first);
        if (n == 3 || (alive && n == 2)) next.insert(kv.first);
    }
    return next;
}

void printBoard(const set<Cell>& live) {
    if (live.empty()) { cout << "(empty)\n"; return; }
    int r0 = INT_MAX, r1 = INT_MIN, c0 = INT_MAX, c1 = INT_MIN;
    for (auto& c : live) {
        r0 = min(r0, c.first); r1 = max(r1, c.first);
        c0 = min(c0, c.second); c1 = max(c1, c.second);
    }
    for (int r = r0; r <= r1; r++) {
        for (int c = c0; c <= c1; c++)
            cout << (live.count({r, c}) ? '*' : '.');
        cout << '\n';
    }
}

int main() {
    set<Cell> live = {{1, 0}, {1, 1}, {1, 2}}; // horizontal blinker
    int steps = 2;
    cout << "Step 0:\n"; printBoard(live);
    for (int s = 1; s <= steps; s++) {
        live = step(live);
        cout << "Step " << s << ":\n";
        printBoard(live);
    }
    return 0;
}
