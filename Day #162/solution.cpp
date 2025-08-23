// Day 162: Shortest unique prefix via trie. Each node stores pass-through count;
// the shortest prefix with count 1 is unique. Time O(total chars), Space O(same).
#include <bits/stdc++.h>
using namespace std;

struct Node { int count = 0; map<char, Node*> child; };

int main() {
    vector<string> words = {"dog", "cat", "apple", "apricot", "fish"};
    Node* root = new Node();
    for (auto& w : words) {
        Node* cur = root;
        for (char c : w) {
            if (!cur->child.count(c)) cur->child[c] = new Node();
            cur = cur->child[c];
            cur->count++;
        }
    }
    for (auto& w : words) {
        Node* cur = root;
        string prefix;
        for (char c : w) {
            cur = cur->child[c];
            prefix += c;
            if (cur->count == 1) break;
        }
        cout << prefix << "\n";
    }
    return 0;
}
