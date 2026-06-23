// Deque ("quack") from three stacks: L (top=leftmost), R (top=rightmost), T temp.
// On empty side, move half the other stack across (T as transient buffer) => O(1) amortized.
#include <bits/stdc++.h>
using namespace std;

struct Quack {
    vector<int> L, R, T; // T only used transiently during rebalance

    void push(int x) { L.push_back(x); } // add to LEFT end

    int pop() { // remove LEFT end
        if (L.empty()) rebalanceLfromR();
        int v = L.back(); L.pop_back(); return v;
    }

    int pull() { // remove RIGHT end
        if (R.empty()) rebalanceRfromL();
        int v = R.back(); R.pop_back(); return v;
    }

    void rebalanceLfromR() {
        int size = R.size();
        int half = size / 2; if (half == 0) half = 1; // elements moved to L
        int keep = size - half;
        for (int k = 0; k < keep; ++k) { T.push_back(R.back()); R.pop_back(); }
        while (!R.empty()) { L.push_back(R.back()); R.pop_back(); }
        while (!T.empty()) { R.push_back(T.back()); T.pop_back(); }
    }

    void rebalanceRfromL() {
        int size = L.size();
        int half = size / 2; if (half == 0) half = 1; // elements moved to R
        int keep = size - half;
        for (int k = 0; k < keep; ++k) { T.push_back(L.back()); L.pop_back(); }
        while (!L.empty()) { R.push_back(L.back()); L.pop_back(); }
        while (!T.empty()) { L.push_back(T.back()); T.pop_back(); }
    }
};

int main() {
    Quack q;
    q.push(1); q.push(2); q.push(3); // left-to-right: 3,2,1
    cout << q.pop() << "\n";   // 3
    cout << q.pull() << "\n";  // 1
    q.push(4);                 // now 4,2
    cout << q.pop() << "\n";   // 4
    cout << q.pull() << "\n";  // 2
    return 0;
}
