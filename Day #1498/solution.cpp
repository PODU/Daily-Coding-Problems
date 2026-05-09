// Day 1498: Conway's Game of Life on an infinite board using a set of live
// (row,col) cells; per step tally neighbor counts over live cells.
// Time O(L) per step (L = live cells), Space O(L).
#include <iostream>
#include <set>
#include <map>
#include <string>
#include <climits>
#include <vector>
#include <utility>
using namespace std;

typedef pair<int,int> Cell;

struct GameOfLife {
    set<Cell> live;
    GameOfLife(vector<Cell> cells) {
        for (size_t i = 0; i < cells.size(); i++) live.insert(cells[i]);
    }
    void step() {
        map<Cell,int> counts;
        for (set<Cell>::iterator it = live.begin(); it != live.end(); ++it) {
            int r = it->first, c = it->second;
            for (int dr = -1; dr <= 1; dr++)
                for (int dc = -1; dc <= 1; dc++)
                    if (dr || dc) counts[make_pair(r + dr, c + dc)]++;
        }
        set<Cell> next;
        for (map<Cell,int>::iterator it = counts.begin(); it != counts.end(); ++it) {
            int n = it->second;
            bool alive = live.count(it->first) > 0;
            if (n == 3 || (alive && n == 2)) next.insert(it->first);
        }
        live = next;
    }
    void print() {
        if (live.empty()) { cout << "(empty)\n"; return; }
        int minR = INT_MAX, maxR = INT_MIN, minC = INT_MAX, maxC = INT_MIN;
        for (set<Cell>::iterator it = live.begin(); it != live.end(); ++it) {
            minR = min(minR, it->first);  maxR = max(maxR, it->first);
            minC = min(minC, it->second); maxC = max(maxC, it->second);
        }
        for (int r = minR; r <= maxR; r++) {
            string row;
            for (int c = minC; c <= maxC; c++)
                row += live.count(make_pair(r, c)) ? '*' : '.';
            cout << row << "\n";
        }
    }
};

int main() {
    vector<Cell> cells;
    cells.push_back(make_pair(0, 1));
    cells.push_back(make_pair(1, 1));
    cells.push_back(make_pair(2, 1));
    GameOfLife game(cells);
    int steps = 2;
    cout << "Step 0:\n"; game.print();
    for (int s = 1; s <= steps; s++) {
        game.step();
        cout << "Step " << s << ":\n";
        game.print();
    }
    return 0;
}
