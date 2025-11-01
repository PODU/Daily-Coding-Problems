// Boggle solver: build a trie from the dictionary, DFS each cell over 8 neighbors
// (each cell used once per path), collect words present in the trie.
// Time: O(cells * 8^L) bounded by trie depth; Space: O(total dictionary chars).
#include <bits/stdc++.h>
using namespace std;

struct Trie {
    Trie* next[26] = {};
    bool word = false;
};

int R, C;
vector<vector<char>> board;
set<string> found;

void dfs(int r, int c, Trie* node, string& cur, vector<vector<bool>>& used) {
    if (r < 0 || r >= R || c < 0 || c >= C || used[r][c]) return;
    char ch = board[r][c];
    Trie* nxt = node->next[ch - 'a'];
    if (!nxt) return;
    cur.push_back(ch);
    used[r][c] = true;
    if (nxt->word) found.insert(cur);
    for (int dr = -1; dr <= 1; dr++)
        for (int dc = -1; dc <= 1; dc++)
            if (dr || dc) dfs(r + dr, c + dc, nxt, cur, used);
    used[r][c] = false;
    cur.pop_back();
}

int main() {
    board = {{'o','a','a','n'},{'e','t','a','e'},{'i','h','k','r'},{'i','f','l','v'}};
    vector<string> dict = {"oath","pea","eat","rain"};
    R = board.size(); C = board[0].size();

    Trie* root = new Trie();
    for (auto& w : dict) {
        Trie* node = root;
        for (char ch : w) {
            if (!node->next[ch - 'a']) node->next[ch - 'a'] = new Trie();
            node = node->next[ch - 'a'];
        }
        node->word = true;
    }

    vector<vector<bool>> used(R, vector<bool>(C, false));
    string cur;
    for (int r = 0; r < R; r++)
        for (int c = 0; c < C; c++)
            dfs(r, c, root, cur, used);

    cout << "[";
    bool first = true;
    for (auto& w : found) { if (!first) cout << ", "; cout << w; first = false; }
    cout << "]" << endl;
    return 0;
}
