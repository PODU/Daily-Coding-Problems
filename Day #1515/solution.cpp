// Sort number-strings by comparator a+b > b+a (largest concat first), join; handle all-zeros.
// Time: O(n log n * L) comparisons, Space: O(n).
#include <bits/stdc++.h>
using namespace std;

string largestNumber(vector<int> nums) {
    vector<string> s;
    for (int x : nums) s.push_back(to_string(x));
    sort(s.begin(), s.end(), [](const string& a, const string& b) {
        return a + b > b + a;
    });
    if (s[0] == "0") return "0";
    string res;
    for (auto& t : s) res += t;
    return res;
}

int main() {
    cout << largestNumber({10, 7, 76, 415}) << endl;
    return 0;
}
