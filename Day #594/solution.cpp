// Day 594: Max number of non-overlapping dictionary words packable on a letter board.
// Approach: for each word enumerate all adjacency placements (DFS) as cell bitmasks,
// then backtracking max set-packing to pick the most pairwise-disjoint placements.
// Time: O(words * board * 4^L) to enumerate + exponential packing on small boards.
#include <bits/stdc++.h>
using namespace std;

int R, C;
vector<vector<char>> board;

void findWord(const string& w, int idx, int r, int c, long long mask,
              vector<long long>& out) {
    if (r < 0 || c < 0 || r >= R || c >= C) return;
    int bit = r * C + c;
    if (mask & (1LL << bit)) return;
    if (board[r][c] != w[idx]) return;
    mask |= (1LL << bit);
    if (idx == (int)w.size() - 1) { out.push_back(mask); return; }
    findWord(w, idx + 1, r + 1, c, mask, out);
    findWord(w, idx + 1, r - 1, c, mask, out);
    findWord(w, idx + 1, r, c + 1, mask, out);
    findWord(w, idx + 1, r, c - 1, mask, out);
}

int best;
void pack(const vector<long long>& placements, int i, long long used, int cnt) {
    best = max(best, cnt);
    for (int j = i; j < (int)placements.size(); ++j) {
        if (!(placements[j] & used))
            pack(placements, j + 1, used | placements[j], cnt + 1);
    }
}

int maxWords(const vector<string>& dict) {
    vector<long long> placements;
    for (const string& w : dict) {
        vector<long long> masks;
        for (int r = 0; r < R; ++r)
            for (int c = 0; c < C; ++c)
                findWord(w, 0, r, c, 0, masks);
        sort(masks.begin(), masks.end());
        masks.erase(unique(masks.begin(), masks.end()), masks.end());
        for (auto m : masks) placements.push_back(m);
    }
    best = 0;
    pack(placements, 0, 0, 0);
    return best;
}

int main() {
    board = {{'e', 'a', 'n'}, {'t', 't', 'i'}, {'a', 'r', 'a'}};
    R = board.size(); C = board[0].size();
    vector<string> dict = {"eat", "rain", "in", "rat"};
    cout << maxWords(dict) << endl; // 3
    return 0;
}
