// Day 1122 - Ghost: winning starting letters for player 1 under optimal play
// Trie + minimax over prefixes. A mover loses if every letter completes a word
// or leads to a winning position for the opponent. Time: O(total letters).
#include <bits/stdc++.h>
using namespace std;

struct TrieNode {
    map<char, TrieNode*> children;
    bool isWord = false;
};

void insert(TrieNode* root, const string& w) {
    TrieNode* node = root;
    for (char ch : w) {
        if (!node->children.count(ch)) node->children[ch] = new TrieNode();
        node = node->children[ch];
    }
    node->isWord = true;
}

bool canWin(TrieNode* node) {
    for (auto& [ch, child] : node->children) {
        if (child->isWord) continue;
        if (!canWin(child)) return true;
    }
    return false;
}

int main() {
    vector<string> words = {"cat", "calf", "dog", "bear"};
    TrieNode* root = new TrieNode();
    for (auto& w : words) insert(root, w);

    vector<char> res;
    for (auto& [ch, child] : root->children)
        if (!child->isWord && !canWin(child)) res.push_back(ch);
    sort(res.begin(), res.end());

    cout << "Winning starting letters:";
    for (char c : res) cout << " " << c;
    cout << "\n";
    return 0;
}
