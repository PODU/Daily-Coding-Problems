// 8-puzzle solver via A* with Manhattan-distance heuristic (admissible -> optimal solution).
// State = 9-int board (blank=0); explore by sliding a tile into the blank. Exponential worst case; Manhattan prunes heavily.
#include <iostream>
#include <vector>
#include <array>
#include <queue>
#include <unordered_map>
#include <algorithm>
#include <cstdlib>
using namespace std;

typedef array<int, 9> Board;

int manhattan(const Board& b) {
    int d = 0;
    for (int i = 0; i < 9; i++) {
        int v = b[i];
        if (v == 0) continue;
        int gr = (v - 1) / 3, gc = (v - 1) % 3;
        int r = i / 3, c = i % 3;
        d += abs(gr - r) + abs(gc - c);
    }
    return d;
}

string key(const Board& b) { string s; for (int x : b) s += char('0' + x); return s; }

int main() {
    Board start = {1, 2, 3, 4, 0, 6, 7, 5, 8};
    Board goal  = {1, 2, 3, 4, 5, 6, 7, 8, 0};

    unordered_map<string, int> g;
    unordered_map<string, string> parent;
    auto cmp = [](const pair<int, Board>& a, const pair<int, Board>& b) { return a.first > b.first; };
    priority_queue<pair<int, Board>, vector<pair<int, Board>>, decltype(cmp)> pq(cmp);

    g[key(start)] = 0;
    pq.push({manhattan(start), start});
    int dr[] = {-1, 1, 0, 0}, dc[] = {0, 0, -1, 1};

    while (!pq.empty()) {
        auto top = pq.top(); pq.pop();
        int f = top.first; Board cur = top.second;
        if (cur == goal) break;
        string ck = key(cur);
        int cg = g[ck];
        if (f > cg + manhattan(cur)) continue; // stale entry
        int z = int(find(cur.begin(), cur.end(), 0) - cur.begin());
        int zr = z / 3, zc = z % 3;
        for (int k = 0; k < 4; k++) {
            int nr = zr + dr[k], nc = zc + dc[k];
            if (nr < 0 || nr >= 3 || nc < 0 || nc >= 3) continue;
            int nz = nr * 3 + nc;
            Board nb = cur;
            swap(nb[z], nb[nz]);
            string nk = key(nb);
            int ng = cg + 1;
            if (!g.count(nk) || ng < g[nk]) {
                g[nk] = ng;
                parent[nk] = ck;
                pq.push({ng + manhattan(nb), nb});
            }
        }
    }

    // Reconstruct path goal -> start, then reverse.
    vector<string> path;
    string k = key(goal);
    path.push_back(k);
    while (k != key(start)) { k = parent[k]; path.push_back(k); }
    reverse(path.begin(), path.end());

    int moves = int(path.size()) - 1;
    cout << "Solved in " << moves << " moves\n";
    cout << "Goal board:\n";
    string gb = path.back();
    for (int i = 0; i < 9; i++)
        cout << gb[i] << (i % 3 == 2 ? "\n" : " ");
    return 0;
}
