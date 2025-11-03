// Maximum XOR of two elements using a binary trie, greedily pick opposite bit.
// Time O(n*32), Space O(n*32).
#include <iostream>
#include <vector>
using namespace std;

struct Trie {
    Trie* child[2] = {nullptr, nullptr};
};

void insert(Trie* root, int num) {
    Trie* cur = root;
    for (int b = 31; b >= 0; --b) {
        int bit = (num >> b) & 1;
        if (!cur->child[bit]) cur->child[bit] = new Trie();
        cur = cur->child[bit];
    }
}

int maxXor(const vector<int>& nums) {
    Trie* root = new Trie();
    for (int x : nums) insert(root, x);
    int best = 0;
    for (int x : nums) {
        Trie* cur = root;
        int cur_xor = 0;
        for (int b = 31; b >= 0; --b) {
            int bit = (x >> b) & 1;
            int want = bit ^ 1;
            if (cur->child[want]) {
                cur_xor |= (1 << b);
                cur = cur->child[want];
            } else {
                cur = cur->child[bit];
            }
        }
        best = max(best, cur_xor);
    }
    return best;
}

int main() {
    vector<int> nums = {3, 10, 5, 25, 2, 8};
    cout << maxXor(nums) << "\n";
    return 0;
}
