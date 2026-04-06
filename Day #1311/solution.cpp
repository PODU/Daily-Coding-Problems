// Quack (deque) via three stacks. On underflow of one end, rebalance by moving
// half the elements through the third stack -> O(1) amortized, O(1) extra memory.
#include <bits/stdc++.h>
using namespace std;

struct Quack {
    vector<int> L, R, T; // left stack (top=leftmost), right (top=rightmost), temp

    // Move half of src to dst, keeping the other half in src, using temp.
    void rebalance(vector<int>& src, vector<int>& dst, vector<int>& tmp) {
        int n = src.size(), k = n / 2;        // k elements stay in src
        for (int i = 0; i < k; i++)     { tmp.push_back(src.back()); src.pop_back(); }
        for (int i = 0; i < n - k; i++) { dst.push_back(src.back()); src.pop_back(); }
        for (int i = 0; i < k; i++)     { src.push_back(tmp.back()); tmp.pop_back(); }
    }
    void push(int x) { L.push_back(x); }                 // add to left
    int pop()  { if (L.empty()) rebalance(R, L, T); int v = L.back(); L.pop_back(); return v; }
    int pull() { if (R.empty()) rebalance(L, R, T); int v = R.back(); R.pop_back(); return v; }
};

int main() {
    Quack q;
    q.push(1); q.push(2); q.push(3); // list (left->right): 3 2 1
    cout << q.pop()  << "\n"; // 3
    cout << q.pull() << "\n"; // 1
    q.push(4);                       // list: 4 2
    cout << q.pull() << "\n"; // 2
    cout << q.pop()  << "\n"; // 4
    return 0;
}
