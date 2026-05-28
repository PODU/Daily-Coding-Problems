// Shortest unique prefix via Trie storing char frequency counts. O(total chars) time & space.
#include <bits/stdc++.h>
using namespace std;

struct Node { map<char, Node*> ch; int cnt = 0; };

int main() {
    vector<string> words = {"dog", "cat", "apple", "apricot", "fish"};
    Node* root = new Node();
    for (auto& w : words) {
        Node* cur = root;
        for (char c : w) {
            if (!cur->ch.count(c)) cur->ch[c] = new Node();
            cur = cur->ch[c];
            cur->cnt++;
        }
    }
    for (auto& w : words) {
        Node* cur = root;
        string pre;
        for (char c : w) {
            cur = cur->ch[c];
            pre += c;
            if (cur->cnt == 1) break;
        }
        cout << pre << "\n";
    }
    return 0;
}
