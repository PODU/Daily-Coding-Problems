// Shortest unique prefix via trie of char frequency counts.
// Walk each word until count==1. Time O(total chars), Space O(total chars).
#include <bits/stdc++.h>
using namespace std;

struct Node { int cnt = 0; unordered_map<char, Node*> next; };

void insert(Node* root, const string& w) {
    Node* cur = root;
    for (char c : w) {
        if (!cur->next.count(c)) cur->next[c] = new Node();
        cur = cur->next[c];
        cur->cnt++;
    }
}

string shortestPrefix(Node* root, const string& w) {
    Node* cur = root;
    string pre;
    for (char c : w) {
        cur = cur->next[c];
        pre += c;
        if (cur->cnt == 1) break;
    }
    return pre;
}

int main() {
    vector<string> words = {"dog", "cat", "apple", "apricot", "fish"};
    Node* root = new Node();
    for (auto& w : words) insert(root, w);
    cout << "[";
    for (size_t i = 0; i < words.size(); ++i) {
        if (i) cout << ", ";
        cout << shortestPrefix(root, words[i]);
    }
    cout << "]" << endl;
    return 0;
}
