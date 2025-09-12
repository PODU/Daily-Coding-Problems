// Day 259: Ghost word game. Build a trie of the dictionary. A starting letter is
// guaranteed safe for player 1 iff every word in its subtree has even length, so
// the opponent is always the one forced to complete a word. Trie DFS, O(total chars).
#include <bits/stdc++.h>
using namespace std;

struct T { map<char, T*> kids; bool word = false; };

void insert(T* root, const string& w) {
    T* n = root;
    for (char c : w) { if (!n->kids.count(c)) n->kids[c] = new T(); n = n->kids[c]; }
    n->word = true;
}

// returns false if any word in this subtree ends at an odd depth (player-1 would complete it)
bool allEven(T* n, int depth) {
    if (n->word && depth % 2 != 0) return false;
    for (auto& kv : n->kids) if (!allEven(kv.second, depth + 1)) return false;
    return true;
}

int main() {
    vector<string> words = {"cat", "calf", "dog", "bear"};
    T* root = new T();
    for (auto& w : words) insert(root, w);

    string res; // map iterates in sorted key order
    for (auto& kv : root->kids) if (allEven(kv.second, 1)) res += kv.first;
    cout << res << endl;
}
