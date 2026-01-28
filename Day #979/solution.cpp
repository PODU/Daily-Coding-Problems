// Conway's Game of Life on an infinite board: store live cells in a set; each step
// tally neighbor counts only around live cells, then apply the 4 rules.
// Time: O(L) per step (L live cells); Space: O(L). Printing bounds to the live region.
#include <bits/stdc++.h>
using namespace std;

struct GameOfLife {
    set<pair<int,int>> live;

    GameOfLife(const vector<pair<int,int>>& cells) {
        for (size_t i = 0; i < cells.size(); i++) live.insert(cells[i]);
    }

    void step() {
        map<pair<int,int>, int> counts;
        for (set<pair<int,int> >::iterator it = live.begin(); it != live.end(); ++it) {
            int r = it->first, c = it->second;
            for (int dr = -1; dr <= 1; dr++)
                for (int dc = -1; dc <= 1; dc++)
                    if (dr || dc) counts[make_pair(r + dr, c + dc)]++;
        }
        set<pair<int,int>> next;
        for (map<pair<int,int>, int>::iterator it = counts.begin(); it != counts.end(); ++it) {
            bool alive = live.count(it->first) != 0;
            int cnt = it->second;
            if (cnt == 3 || (alive && cnt == 2)) next.insert(it->first);
        }
        live = next;
    }

    void print(int stepNo) const {
        cout << "Step " << stepNo << ":\n";
        if (live.empty()) { cout << "(empty)\n\n"; return; }
        int minR = INT_MAX, maxR = INT_MIN, minC = INT_MAX, maxC = INT_MIN;
        for (set<pair<int,int> >::const_iterator it = live.begin(); it != live.end(); ++it) {
            minR = min(minR, it->first);  maxR = max(maxR, it->first);
            minC = min(minC, it->second); maxC = max(maxC, it->second);
        }
        for (int r = minR; r <= maxR; r++) {
            string row;
            for (int c = minC; c <= maxC; c++)
                row += live.count(make_pair(r, c)) ? '*' : '.';
            cout << row << "\n";
        }
        cout << "\n";
    }
};

int main() {
    vector<pair<int,int>> cells;
    cells.push_back(make_pair(0, 0));
    cells.push_back(make_pair(0, 1));
    cells.push_back(make_pair(0, 2)); // horizontal blinker
    GameOfLife g(cells);
    int steps = 2;
    g.print(0);
    for (int s = 1; s <= steps; s++) {
        g.step();
        g.print(s);
    }
    return 0;
}
