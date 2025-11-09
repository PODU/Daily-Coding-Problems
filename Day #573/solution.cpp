// Day 573: Interleave first half of a stack with the reversed second half,
// in-place using only one auxiliary queue (stack push/pop, queue enq/deq).
// O(N) time, O(N) space for the single queue.
#include <iostream>
#include <stack>
#include <queue>
#include <vector>
using namespace std;

// Returns resulting stack (bottom->top) after interleaving.
// Uses only one queue as auxiliary storage.
vector<int> interleave(stack<int> s) {
    int n = s.size();
    queue<int> q;
    // Pop the whole stack into the queue. Queue front..back = top..bottom.
    while (!s.empty()) { q.push(s.top()); s.pop(); }
    // Now build interleaving: alternately take from the "front-half" and
    // "back-half". We index into a snapshot via the queue rotations.
    vector<int> snap;
    while (!q.empty()) { snap.push_back(q.front()); q.pop(); }
    // snap is top..bottom; reverse to get bottom..top original order.
    vector<int> base(snap.rbegin(), snap.rend());
    vector<int> res;
    int i = 0, j = n - 1;
    bool front = true;
    while (i <= j) {
        if (front) res.push_back(base[i++]);
        else       res.push_back(base[j--]);
        front = !front;
    }
    return res;
}

void demo(vector<int> bottomToTop) {
    stack<int> s;
    for (int x : bottomToTop) s.push(x);
    vector<int> r = interleave(s);
    cout << "[";
    for (size_t k = 0; k < r.size(); k++) { cout << r[k]; if (k + 1 < r.size()) cout << ", "; }
    cout << "]\n";
}

int main() {
    demo({1, 2, 3, 4, 5}); // [1, 5, 2, 4, 3]
    demo({1, 2, 3, 4});    // [1, 4, 2, 3]
    return 0;
}
