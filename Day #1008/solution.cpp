// Maximum XOR of two elements using a binary trie (bits high->low), greedy opposite bit.
// Time O(n*B), Space O(n*B), B = 32.
#include <bits/stdc++.h>
using namespace std;

struct Trie {
    Trie* child[2] = {nullptr, nullptr};
};

const int B = 31; // bits 30..0 cover values up to ~2^31

int maximumXOR(const vector<int>& nums) {
    Trie* root = new Trie();
    int best = 0;
    for (int x : nums) {
        Trie* ins = root;
        Trie* cur = root;
        int curXor = 0;
        for (int b = B - 1; b >= 0; --b) {
            int bit = (x >> b) & 1;
            // insert
            if (!ins->child[bit]) ins->child[bit] = new Trie();
            ins = ins->child[bit];
            // query best partner (go opposite)
            int want = bit ^ 1;
            if (cur->child[want]) { curXor |= (1 << b); cur = cur->child[want]; }
            else if (cur->child[bit]) cur = cur->child[bit];
        }
        best = max(best, curXor);
    }
    return best;
}

int main() {
    vector<int> nums = {3, 10, 5, 25, 2, 8};
    cout << maximumXOR(nums) << endl; // 28
    return 0;
}
