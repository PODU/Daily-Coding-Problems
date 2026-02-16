// reduce/fold: apply combiner left-to-right starting from init. Time O(n), Space O(1).
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
    int s = reduce<int, int>(lst, [](int a, int b) { return a + b; }, 0);
    cout << s << "\n"; // 15
}
