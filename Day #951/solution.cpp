// Day 951: interleave first half of a stack with the reversed second half,
// in place using only one auxiliary queue (push/pop, enqueue/dequeue).
// Time O(n^2) due to rotations, Space O(1) extra besides the queue.
#include <bits/stdc++.h>
using namespace std;

// stack represented with top at back(); queue is a std::deque used as a queue.
vector<long long> interleave(vector<long long> st) {
    deque<long long> q;
    // Move everything to queue so the queue front->rear holds bottom->top order.
    while (!st.empty()) { q.push_back(st.back()); st.pop_back(); } // q: top..bottom
    while (!q.empty())  { st.push_back(q.front()); q.pop_front(); } // st bottom..top: top..bottom
    while (!st.empty()) { q.push_back(st.back()); st.pop_back(); } // q: a0..a_{n-1}
    // Build target by pulling front then back of the queue alternately.
    while (!q.empty()) {
        st.push_back(q.front()); q.pop_front();      // front (low index)
        long long m = (long long)q.size();
        if (m == 0) break;
        for (long long i = 0; i < m - 1; ++i) { q.push_back(q.front()); q.pop_front(); }
        st.push_back(q.front()); q.pop_front();      // back (high index)
    }
    return st; // bottom..top
}

void show(const vector<long long>& v) {
    cout << "[";
    for (size_t i = 0; i < v.size(); ++i) cout << v[i] << (i + 1 < v.size() ? ", " : "");
    cout << "]\n";
}

int main() {
    show(interleave({1, 2, 3, 4, 5})); // [1, 5, 2, 4, 3]
    show(interleave({1, 2, 3, 4}));    // [1, 4, 2, 3]
    return 0;
}
