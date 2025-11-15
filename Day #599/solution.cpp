// Day 599: Ghost game - find starting letters that guarantee player 1 a win.
// Approach: build a trie, minimax over it (landing on a word loses). Time O(total chars), Space O(trie).
#include <bits/stdc++.h>
using namespace std;

struct Trie {
    map<char, Trie*> ch;
    bool isWord = false;
};

void insert(Trie* root, const string& w) {
    Trie* node = root;
    for (char c : w) {
        if (!node->ch.count(c)) node->ch[c] = new Trie();
        node = node->ch[c];
    }
    node->isWord = true;
}

// True if the player to move from `node` can force the opponent to lose.
bool moverWins(Trie* node) {
    for (auto& kv : node->ch) {
        Trie* child = kv.second;
        if (child->isWord) continue;        // completing a word => mover loses
        if (!moverWins(child)) return true; // opponent loses => mover wins
    }
    return false;
}

int main() {
    vector<string> dict = {"cat", "calf", "dog", "bear"};
    Trie* root = new Trie();
    for (auto& w : dict) insert(root, w);

    vector<char> winning;
    for (auto& kv : root->ch)
        if (!kv.second->isWord && !moverWins(kv.second)) winning.push_back(kv.first);

    for (size_t i = 0; i < winning.size(); i++)
        cout << winning[i] << (i + 1 < winning.size() ? " " : "\n");
    if (winning.empty()) cout << "\n";
    return 0;
}
