// Ghost word game: trie + game theory. canWin(node)=mover can force win.
// Winning start c: child not a word AND opponent (canWin(child)) cannot win.
// Time O(total chars), Space O(total chars).
// NOTE: README sample shows only "b" but "c" is also winning.
#include <bits/stdc++.h>
using namespace std;

struct Node {
    map<char, Node*> ch;
    bool isWord = false;
};

void insert(Node* root, const string& w) {
    Node* cur = root;
    for (char c : w) {
        if (!cur->ch.count(c)) cur->ch[c] = new Node();
        cur = cur->ch[c];
    }
    cur->isWord = true;
}

// can the player about to move from this prefix force a win?
bool canWin(Node* node) {
    for (map<char, Node*>::iterator it = node->ch.begin(); it != node->ch.end(); ++it) {
        Node* child = it->second;
        if (child->isWord) continue;        // choosing it completes a word -> mover loses
        if (!canWin(child)) return true;    // opponent then loses
    }
    return false;
}

int main() {
    vector<string> dict = {"cat", "calf", "dog", "bear"};
    Node* root = new Node();
    for (auto& w : dict) insert(root, w);

    vector<char> wins;
    for (map<char, Node*>::iterator it = root->ch.begin(); it != root->ch.end(); ++it) {
        if (!it->second->isWord && !canWin(it->second)) wins.push_back(it->first);
    }
    sort(wins.begin(), wins.end());

    for (size_t i = 0; i < wins.size(); ++i) {
        if (i) cout << ' ';
        cout << wins[i];
    }
    cout << '\n';
    return 0;
}
