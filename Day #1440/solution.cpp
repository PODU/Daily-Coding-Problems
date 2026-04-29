// Day 1440: Ternary search tree with insert and search.
// Approach: each node stores a char + left/mid/right; mid advances the word.
// Time: insert/search O(L * log A) avg, Space: O(total chars).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    char c;
    bool isEnd = false;
    Node *left = nullptr, *mid = nullptr, *right = nullptr;
    Node(char ch) : c(ch) {}
};

Node* insert(Node* root, const string& word, int i) {
    if (i >= (int)word.size()) return root;
    char ch = word[i];
    if (!root) root = new Node(ch);
    if (ch < root->c) {
        root->left = insert(root->left, word, i);
    } else if (ch > root->c) {
        root->right = insert(root->right, word, i);
    } else {
        if (i + 1 == (int)word.size()) root->isEnd = true;
        else root->mid = insert(root->mid, word, i + 1);
    }
    return root;
}

bool search(Node* root, const string& word, int i) {
    if (!root || i >= (int)word.size()) return false;
    char ch = word[i];
    if (ch < root->c) return search(root->left, word, i);
    if (ch > root->c) return search(root->right, word, i);
    if (i + 1 == (int)word.size()) return root->isEnd;
    return search(root->mid, word, i + 1);
}

int main() {
    Node* root = nullptr;
    for (string w : {"code", "cob", "be", "ax", "war", "we"})
        root = insert(root, w, 0);
    cout << boolalpha;
    cout << search(root, "code", 0) << endl; // true
    cout << search(root, "cob", 0) << endl;  // true
    cout << search(root, "we", 0) << endl;   // true
    cout << search(root, "cod", 0) << endl;  // false
    cout << search(root, "cat", 0) << endl;  // false
    return 0;
}
