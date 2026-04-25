// Day 1426: Maximum XOR of any two elements in an array.
// Approach: Binary trie of bits; for each number greedily pick opposite bit.
// Time: O(n * B), Space: O(n * B) where B = number of bits (32).
#include <bits/stdc++.h>
using namespace std;

struct Trie {
    Trie* child[2] = {nullptr, nullptr};
};

int findMaxXOR(const vector<int>& nums) {
    Trie* root = new Trie();
    int maxXor = 0;
    const int BITS = 31;
    for (int num : nums) {
        // insert
        Trie* node = root;
        for (int b = BITS; b >= 0; --b) {
            int bit = (num >> b) & 1;
            if (!node->child[bit]) node->child[bit] = new Trie();
            node = node->child[bit];
        }
        // query best partner
        node = root;
        int cur = 0;
        for (int b = BITS; b >= 0; --b) {
            int bit = (num >> b) & 1;
            if (node->child[1 - bit]) {
                cur |= (1 << b);
                node = node->child[1 - bit];
            } else {
                node = node->child[bit];
            }
        }
        maxXor = max(maxXor, cur);
    }
    return maxXor;
}

int main() {
    vector<int> nums = {3, 10, 5, 25, 2, 8};
    cout << findMaxXOR(nums) << endl; // 28 (5 ^ 25)
    return 0;
}
