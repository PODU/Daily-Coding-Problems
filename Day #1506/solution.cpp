// Maximum XOR of two elements using a binary (bitwise prefix) trie over 32 bits.
// Insert each number's bits, query best complement. Time O(n*32), Space O(n*32).
#include <bits/stdc++.h>
using namespace std;

struct Trie {
    Trie* child[2] = {nullptr, nullptr};
};

void insertNum(Trie* root, int num) {
    Trie* node = root;
    for (int i = 31; i >= 0; --i) {
        int b = (num >> i) & 1;
        if (!node->child[b]) node->child[b] = new Trie();
        node = node->child[b];
    }
}

int queryBest(Trie* root, int num) {
    Trie* node = root;
    int best = 0;
    for (int i = 31; i >= 0; --i) {
        int b = (num >> i) & 1;
        int want = b ^ 1;
        if (node->child[want]) {
            best |= (1 << i);
            node = node->child[want];
        } else {
            node = node->child[b];
        }
    }
    return best;
}

int findMaximumXOR(vector<int>& nums) {
    Trie* root = new Trie();
    int best = 0;
    for (int x : nums) {
        insertNum(root, x);
        best = max(best, queryBest(root, x));
    }
    return best;
}

int main() {
    vector<int> nums = {3, 10, 5, 25, 2, 8};
    cout << findMaximumXOR(nums) << "\n";
    return 0;
}
