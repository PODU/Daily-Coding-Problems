// Maximum XOR of any two elements using a binary trie of bits.
// Insert each number, greedily pick opposite bit per number. O(n*bits) time, O(n*bits) space.
#include <iostream>
#include <vector>
using namespace std;

struct Trie {
    Trie* child[2] = {nullptr, nullptr};
};

const int BITS = 32;

int findMaxXOR(const vector<int>& nums) {
    Trie* root = new Trie();
    for (int x : nums) {
        Trie* node = root;
        for (int i = BITS - 1; i >= 0; --i) {
            int b = (x >> i) & 1;
            if (!node->child[b]) node->child[b] = new Trie();
            node = node->child[b];
        }
    }
    int best = 0;
    for (int x : nums) {
        Trie* node = root;
        int cur = 0;
        for (int i = BITS - 1; i >= 0; --i) {
            int b = (x >> i) & 1;
            int want = b ^ 1;
            if (node->child[want]) {
                cur |= (1 << i);
                node = node->child[want];
            } else {
                node = node->child[b];
            }
        }
        if (cur > best) best = cur;
    }
    return best;
}

int main() {
    vector<int> nums = {3, 10, 5, 25, 2, 8};
    cout << findMaxXOR(nums) << "\n";
    return 0;
}
