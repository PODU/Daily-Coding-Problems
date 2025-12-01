// Next permutation of digits: find pivot, swap with next-larger suffix digit,
// reverse suffix. Time: O(d) digits, Space: O(d).
#include <iostream>
#include <string>
#include <algorithm>
using namespace std;

long long nextPermutation(long long num) {
    string d = to_string(num);
    int n = (int)d.size();
    int i = n - 2;
    while (i >= 0 && d[i] >= d[i + 1]) i--;
    if (i < 0) return num; // already largest permutation
    int j = n - 1;
    while (d[j] <= d[i]) j--;
    swap(d[i], d[j]);
    reverse(d.begin() + i + 1, d.end());
    return stoll(d);
}

int main() {
    cout << nextPermutation(48975) << "\n";
    return 0;
}
