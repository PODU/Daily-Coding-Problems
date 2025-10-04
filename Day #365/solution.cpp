// Day 365: "Quack" deque (push/pop left, pull right) from three stacks.
// l holds the left part (left end on top), r the right part (right end on top);
// tmp is a transient helper used only to re-split when one side empties.
// Each rebalance moves k elems but follows k ops -> O(1) amortized, O(1) extra mem.
#include <bits/stdc++.h>
using namespace std;

struct Quack {
    vector<int> l, r, tmp;

    void push(int x) { l.push_back(x); }   // add to left end

    // Refill an empty `to` side by splitting the full `from` side in half,
    // using tmp. After this, `to` holds the half nearest the shared middle.
    void rebalance(vector<int>& to, vector<int>& from, int toCount) {
        int n = from.size();
        // move the (n-toCount) elements not destined for `to` into tmp
        for (int i = 0; i < n - toCount; i++) { tmp.push_back(from.back()); from.pop_back(); }
        // move the remaining toCount into `to` (reversing -> correct end on top)
        for (int i = 0; i < toCount; i++) { to.push_back(from.back()); from.pop_back(); }
        // restore the rest back into `from`
        while (!tmp.empty()) { from.push_back(tmp.back()); tmp.pop_back(); }
    }

    int pop() {                              // remove left end
        if (l.empty()) rebalance(l, r, (r.size() + 1) / 2);
        int v = l.back(); l.pop_back(); return v;
    }

    int pull() {                             // remove right end
        if (r.empty()) rebalance(r, l, (l.size() + 1) / 2);
        int v = r.back(); r.pop_back(); return v;
    }
};

int main() {
    Quack q;
    q.push(1); q.push(2); q.push(3);     // list L->R: 3 2 1
    cout << q.pop() << "\n";             // 3
    cout << q.pull() << "\n";            // 1
    q.push(4); q.push(5);                // list L->R: 5 4 2
    cout << q.pull() << "\n";            // 2
    cout << q.pop() << "\n";             // 5
    cout << q.pop() << "\n";             // 4
    return 0;
}
