// Day 1326: Implement reduce/fold — fold an array left to right with a combining function and an initial value.
// O(n) calls to the combiner, O(1) extra space.
#include <bits/stdc++.h>
using namespace std;

template <typename T, typename A, typename F>
A reduce(const vector<T>& lst, F combine, A init) {
    A acc = init;
    for (const T& x : lst) acc = combine(acc, x);
    return acc;
}

int main() {
    vector<int> lst = {1, 2, 3, 4, 5};
    int total = reduce(lst, [](int a, int b) { return a + b; }, 0);
    cout << total << endl; // 15
    return 0;
}
