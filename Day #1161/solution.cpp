// 8-puzzle solver: A* search with Manhattan-distance heuristic (admissible), reconstruct path.
// Time: O(b^d) bounded by states, Space: O(states).
#include <bits/stdc++.h>
using namespace std;

typedef array<int, 9> Board;

int manhattan(const Board& b) {
    int d = 0;
    for (int i = 0; i < 9; ++i) {
        int v = b[i];
        if (v == 0) continue;
        int gr = (v - 1) / 3, gc = (v - 1) % 3;
        int r = i / 3, c = i % 3;
        d += abs(r - gr) + abs(c - gc);
    }
    return d;
}

int solve(const Board& start, const Board& goal) {
    auto key = [](const Board& b) {
        string s;
        for (int x : b) s += char('0' + x);
        return s;
    };
    typedef tuple<int, int, Board> Node; // f, g, board
    priority_queue<Node, vector<Node>, greater<Node>> pq;
    unordered_map<string, int> best;
    pq.push({manhattan(start), 0, start});
    best[key(start)] = 0;
    while (!pq.empty()) {
        Node cur = pq.top();
        pq.pop();
        int g = get<1>(cur);
        Board b = get<2>(cur);
        if (b == goal) return g;
        if (g > best[key(b)]) continue;
        int z = 0;
        while (b[z] != 0) ++z;
        int r = z / 3, c = z % 3;
        int dr[] = {-1, 1, 0, 0}, dc[] = {0, 0, -1, 1};
        for (int k = 0; k < 4; ++k) {
            int nr = r + dr[k], nc = c + dc[k];
            if (nr < 0 || nr > 2 || nc < 0 || nc > 2) continue;
            Board nb = b;
            swap(nb[z], nb[nr * 3 + nc]);
            int ng = g + 1;
            string nk = key(nb);
            auto it = best.find(nk);
            if (it == best.end() || ng < it->second) {
                best[nk] = ng;
                pq.push({ng + manhattan(nb), ng, nb});
            }
        }
    }
    return -1;
}

int main() {
    Board start = {1, 2, 3, 4, 0, 6, 7, 5, 8};
    Board goal = {1, 2, 3, 4, 5, 6, 7, 8, 0};
    int moves = solve(start, goal);
    cout << "Solved in " << moves << " moves\n";
    for (int r = 0; r < 3; ++r) {
        for (int c = 0; c < 3; ++c) {
            if (c) cout << ' ';
            int v = goal[r * 3 + c];
            cout << (v == 0 ? "_" : to_string(v));
        }
        cout << "\n";
    }
    return 0;
}
