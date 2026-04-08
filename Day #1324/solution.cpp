// Day 1324: Point-update / range-sum over a 24-element array using a Fenwick (Binary Indexed) Tree.
// update O(log n), query O(log n). 1-indexed internally over fixed size 24.
#include <bits/stdc++.h>
using namespace std;

struct Subscribers {
    int tree[25] = {0};
    void update(int hour, int value) {            // increment index `hour` by value
        for (int i = hour + 1; i <= 24; i += i & (-i)) tree[i] += value;
    }
    int prefix(int hour) {                        // sum of [0, hour]
        int s = 0;
        for (int i = hour + 1; i > 0; i -= i & (-i)) s += tree[i];
        return s;
    }
    int query(int start, int end) {               // inclusive [start, end]
        return prefix(end) - (start > 0 ? prefix(start - 1) : 0);
    }
};

int main() {
    Subscribers s;
    s.update(2, 10);
    s.update(5, 3);
    s.update(5, 7);
    cout << s.query(2, 5) << endl;  // 20
    cout << s.query(0, 23) << endl; // 20
    cout << s.query(3, 4) << endl;  // 0
    return 0;
}
