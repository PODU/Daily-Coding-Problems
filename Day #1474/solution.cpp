// Day 1474: Boggle solver. Trie of dictionary + DFS from each cell over
// 8-adjacent neighbors (no reuse), pruned by trie prefixes.
// Time O(R*C*8^L) worst case; Space O(total dict chars).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    map<char, Node*> next;
    string word;  // non-empty if a word ends here
};

void dfs(vector<vector<char>>& b, int r, int c, Node* node, set<string>& found) {
    char ch = b[r][c];
    if (ch == '*' || !node->next.count(ch)) return;
    Node* nxt = node->next[ch];
    if (!nxt->word.empty()) found.insert(nxt->word);
    b[r][c] = '*';
    for (int dr = -1; dr <= 1; ++dr)
        for (int dc = -1; dc <= 1; ++dc) {
            if (dr == 0 && dc == 0) continue;
            int nr = r + dr, nc = c + dc;
            if (nr >= 0 && nr < (int)b.size() && nc >= 0 && nc < (int)b[0].size())
                dfs(b, nr, nc, nxt, found);
        }
    b[r][c] = ch;
}

int main() {
    vector<vector<char>> board = {
        {'o', 'a', 'a', 'n'},
        {'e', 't', 'a', 'e'},
        {'i', 'h', 'k', 'r'},
        {'i', 'f', 'l', 'v'},
    };
    vector<string> words = {"oath", "pea", "eat", "rain"};

    Node* root = new Node();
    for (auto& w : words) {
        Node* node = root;
        for (char ch : w) {
            if (!node->next.count(ch)) node->next[ch] = new Node();
            node = node->next[ch];
        }
        node->word = w;
    }

    set<string> found;
    for (int r = 0; r < (int)board.size(); ++r)
        for (int c = 0; c < (int)board[0].size(); ++c)
            dfs(board, r, c, root, found);

    cout << "[";
    bool first = true;
    for (auto& w : found) { cout << (first ? "" : ", ") << w; first = false; }
    cout << "]\n";  // [eat, oath]
}
