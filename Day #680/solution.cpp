// Trie with per-node pass counts; shortest unique prefix = up to first node count==1. Time O(total chars).
#include <bits/stdc++.h>
using namespace std;

struct Node {
    map<char, int> next;
    int count = 0;
};

vector<string> shortestUniquePrefixes(const vector<string>& words) {
    vector<Node> trie(1);
    for (const auto& w : words) {
        int cur = 0;
        for (char c : w) {
            if (!trie[cur].next.count(c)) {
                trie[cur].next[c] = trie.size();
                trie.push_back(Node());
            }
            cur = trie[cur].next[c];
            trie[cur].count++;
        }
    }
    vector<string> res;
    for (const auto& w : words) {
        int cur = 0;
        string pre;
        for (char c : w) {
            cur = trie[cur].next[c];
            pre += c;
            if (trie[cur].count == 1) break;
        }
        res.push_back(pre);
    }
    return res;
}

int main() {
    vector<string> words = {"dog", "cat", "apple", "apricot", "fish"};
    auto res = shortestUniquePrefixes(words);
    cout << "[";
    for (size_t i = 0; i < res.size(); ++i)
        cout << res[i] << (i + 1 < res.size() ? ", " : "");
    cout << "]\n";
    return 0;
}
