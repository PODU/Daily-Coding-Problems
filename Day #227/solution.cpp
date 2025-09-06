// Boggle solver: build a Trie of the dictionary, DFS from every cell over 8 neighbours.
// Time: O(cells * 8^L) bounded by Trie; Space: O(dictionary size).
#include <bits/stdc++.h>
using namespace std;

struct Trie {
    Trie* next[26] = {};
    bool word = false;
};

void dfs(vector<vector<char>>& b, int r, int c, Trie* node, string& cur, set<string>& found) {
    if (r < 0 || c < 0 || r >= (int)b.size() || c >= (int)b[0].size() || b[r][c] == '#') return;
    char ch = b[r][c];
    Trie* nx = node->next[ch - 'a'];
    if (!nx) return;
    cur.push_back(ch);
    if (nx->word) found.insert(cur);
    b[r][c] = '#';
    for (int dr = -1; dr <= 1; dr++)
        for (int dc = -1; dc <= 1; dc++)
            if (dr || dc) dfs(b, r + dr, c + dc, nx, cur, found);
    b[r][c] = ch;
    cur.pop_back();
}

vector<string> boggle(vector<vector<char>> board, vector<string>& dict) {
    Trie root;
    for (auto& w : dict) {
        Trie* n = &root;
        for (char ch : w) {
            if (!n->next[ch - 'a']) n->next[ch - 'a'] = new Trie();
            n = n->next[ch - 'a'];
        }
        n->word = true;
    }
    set<string> found;
    string cur;
    for (int r = 0; r < (int)board.size(); r++)
        for (int c = 0; c < (int)board[0].size(); c++)
            dfs(board, r, c, &root, cur, found);
    return vector<string>(found.begin(), found.end());
}

int main() {
    vector<vector<char>> board = {
        {'o', 'a', 'a', 'n'},
        {'e', 't', 'a', 'e'},
        {'i', 'h', 'k', 'r'},
        {'i', 'f', 'l', 'v'}};
    vector<string> dict = {"oath", "pea", "eat", "rain"};
    auto res = boggle(board, dict);
    cout << "[";
    for (size_t i = 0; i < res.size(); i++) {
        cout << "'" << res[i] << "'";
        if (i + 1 < res.size()) cout << ", ";
    }
    cout << "]\n";
}
