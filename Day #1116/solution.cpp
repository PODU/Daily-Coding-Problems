// Day 1116 - Quack: push/pop left, pull right, using three stacks.
// Two stacks L (top=leftmost) and R (top=rightmost); rebalance by splitting the
// other in half via scratch stack T. Amortized O(1) per op, O(1) extra memory.
#include <bits/stdc++.h>
using namespace std;

struct Quack {
    vector<int> L, R, T;

    void push(int x) { L.push_back(x); }

    void rebalanceToL() {
        int m = R.size();
        int rightCount = m / 2;
        for (int i = 0; i < rightCount; ++i) { T.push_back(R.back()); R.pop_back(); }
        while (!R.empty()) { L.push_back(R.back()); R.pop_back(); }
        while (!T.empty()) { R.push_back(T.back()); T.pop_back(); }
    }

    void rebalanceToR() {
        int m = L.size();
        int leftCount = m / 2;
        for (int i = 0; i < leftCount; ++i) { T.push_back(L.back()); L.pop_back(); }
        while (!L.empty()) { R.push_back(L.back()); L.pop_back(); }
        while (!T.empty()) { L.push_back(T.back()); T.pop_back(); }
    }

    int pop() {
        if (L.empty()) rebalanceToL();
        int v = L.back(); L.pop_back(); return v;
    }

    int pull() {
        if (R.empty()) rebalanceToR();
        int v = R.back(); R.pop_back(); return v;
    }
};

int main() {
    Quack q;
    for (int x : {1, 2, 3}) q.push(x);
    cout << "pop: " << q.pop() << "\n";   // 3
    cout << "pull: " << q.pull() << "\n"; // 1
    for (int x : {4, 5}) q.push(x);
    cout << "pull: " << q.pull() << "\n"; // 2
    cout << "pop: " << q.pop() << "\n";   // 5
    cout << "pull: " << q.pull() << "\n"; // 4
    return 0;
}
