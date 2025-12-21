// Day 772: Boggle solver. Trie of dictionary + DFS from each cell over 8 neighbors,
// marking visited. O(W) to build trie; search bounded by trie/board size.
#include <bits/stdc++.h>
using namespace std;

struct Node { Node* ch[26] = {}; bool end = false; };

void insert(Node* root, const string& w) {
    Node* cur = root;
    for (char c : w) { int i = c - 'a'; if (!cur->ch[i]) cur->ch[i] = new Node(); cur = cur->ch[i]; }
    cur->end = true;
}

void dfs(vector<string>& board, int r, int c, Node* node, string& path, set<string>& out) {
    int R = board.size(), C = board[0].size();
    char ch = board[r][c];
    if (ch == '#') return;
    Node* nxt = node->ch[ch - 'a'];
    if (!nxt) return;
    path.push_back(ch);
    if (nxt->end) out.insert(path);
    board[r][c] = '#';
    for (int dr = -1; dr <= 1; dr++)
        for (int dc = -1; dc <= 1; dc++) {
            if (dr == 0 && dc == 0) continue;
            int nr = r + dr, nc = c + dc;
            if (nr >= 0 && nr < R && nc >= 0 && nc < C)
                dfs(board, nr, nc, nxt, path, out);
        }
    board[r][c] = ch;
    path.pop_back();
}

vector<string> solveBoggle(vector<string> board, vector<string>& dict) {
    Node* root = new Node();
    for (auto& w : dict) insert(root, w);
    set<string> out;
    string path;
    for (int r = 0; r < (int)board.size(); r++)
        for (int c = 0; c < (int)board[0].size(); c++)
            dfs(board, r, c, root, path, out);
    return vector<string>(out.begin(), out.end());
}

int main() {
    vector<string> board = {"oaan", "etae", "ihkr", "iflv"};
    vector<string> dict = {"oath", "pea", "eat", "rain"};
    for (auto& w : solveBoggle(board, dict)) cout << w << " ";
    cout << endl; // eat oath
    return 0;
}
