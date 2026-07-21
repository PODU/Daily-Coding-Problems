// Day 1853: Stack with O(1) push/pop/max.
// Keep an auxiliary stack of running maxima alongside the data stack. All ops O(1) time, O(n) space.
#include <bits/stdc++.h>
using namespace std;

class MaxStack {
    vector<int> data, maxs;
public:
    void push(int v) {
        data.push_back(v);
        maxs.push_back(maxs.empty() ? v : max(maxs.back(), v));
    }
    int pop() {
        if (data.empty()) throw runtime_error("empty stack");
        int v = data.back();
        data.pop_back(); maxs.pop_back();
        return v;
    }
    int max() {
        if (maxs.empty()) throw runtime_error("empty stack");
        return maxs.back();
    }
};

int main() {
    MaxStack s;
    s.push(1); s.push(5); s.push(3);
    cout << s.max() << "\n";  // 5
    cout << s.pop() << "\n";  // 3
    cout << s.max() << "\n";  // 5
    cout << s.pop() << "\n";  // 5
    cout << s.max() << "\n";  // 1
    return 0;
}
