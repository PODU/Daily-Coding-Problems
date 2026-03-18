// Trie with pass-through counts; for each word walk down until count==1 to get its shortest unique prefix.
// Time: O(total chars), Space: O(total chars).
#include <bits/stdc++.h>
using namespace std;

struct Trie {
    map<char, Trie*> next;
    int count = 0;
};

void insert(Trie* root, const string& w) {
    Trie* cur = root;
    for (char c : w) {
        if (!cur->next.count(c)) cur->next[c] = new Trie();
        cur = cur->next[c];
        cur->count++;
    }
}

string prefix(Trie* root, const string& w) {
    Trie* cur = root;
    string p;
    for (char c : w) {
        cur = cur->next[c];
        p += c;
        if (cur->count == 1) break;
    }
    return p;
}

int main() {
    vector<string> words = {"dog", "cat", "apple", "apricot", "fish"};
    Trie* root = new Trie();
    for (auto& w : words) insert(root, w);
    cout << "[";
    for (size_t i = 0; i < words.size(); ++i) {
        cout << prefix(root, words[i]);
        if (i + 1 < words.size()) cout << ", ";
    }
    cout << "]" << endl;
    return 0;
}
