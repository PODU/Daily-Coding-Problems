// Max stack with O(1) push/pop/max using an auxiliary stack of running maxima.
// All operations O(1) time; O(n) space.
#include <iostream>
#include <stack>
#include <stdexcept>
using namespace std;

class MaxStack {
    stack<int> data;
    stack<int> maxs;
public:
    void push(int val) {
        data.push(val);
        if (maxs.empty() || val >= maxs.top()) maxs.push(val);
        else maxs.push(maxs.top());
    }
    int pop() {
        if (data.empty()) throw runtime_error("empty");
        int v = data.top();
        data.pop();
        maxs.pop();
        return v;
    }
    int max() {
        if (maxs.empty()) throw runtime_error("empty");
        return maxs.top();
    }
};

int main() {
    MaxStack s;
    for (int v : {1, 5, 3, 9, 2}) s.push(v);
    cout << "max: " << s.max() << '\n';
    cout << "pop: " << s.pop() << '\n';
    cout << "pop: " << s.pop() << '\n';
    cout << "max: " << s.max() << '\n';
    return 0;
}
