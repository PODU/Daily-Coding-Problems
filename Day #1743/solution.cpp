// Left fold (reduce): single linear pass applying combiner to accumulator. O(n) time, O(1) extra space.
#include <bits/stdc++.h>
using namespace std;

template <typename T, typename A, typename F>
A reduce_left(const vector<T>& arr, F comb, A init) {
    A acc = init;
    for (const T& x : arr) acc = comb(acc, x);
    return acc;
}

int main() {
    vector<int> a = {1, 2, 3, 4, 5};
    int s = reduce_left<int, int>(a, [](int acc, int x) { return acc + x; }, 0);
    cout << s << "\n";

    vector<int> b = {1, 2, 3, 4};
    int p = reduce_left<int, int>(b, [](int acc, int x) { return acc * x; }, 1);
    cout << p << "\n";
    return 0;
}
