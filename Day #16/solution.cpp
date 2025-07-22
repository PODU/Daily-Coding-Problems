// Approach: Circular (ring) buffer of size N. record/get_last are O(1); O(N) space.
#include <bits/stdc++.h>
using namespace std;

class OrderLog {
    vector<int> buf;
    int n, count = 0, head = 0; // head = next write position
public:
    OrderLog(int N) : buf(N), n(N) {}
    void record(int order_id) {
        buf[head] = order_id;
        head = (head + 1) % n;
        if (count < n) count++;
    }
    // i is 1-based: get_last(1) is the most recent
    int get_last(int i) {
        int idx = ((head - i) % n + n) % n;
        return buf[idx];
    }
};

int main() {
    OrderLog log(3);
    for (int x : {1, 2, 3, 4, 5}) log.record(x);
    cout << "get_last(1) = " << log.get_last(1) << "\n";
    cout << "get_last(2) = " << log.get_last(2) << "\n";
    cout << "get_last(3) = " << log.get_last(3) << "\n";
    return 0;
}
