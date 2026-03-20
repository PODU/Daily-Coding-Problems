// Fewest insertions for palindrome; lexicographically smallest among minima.
// Interval DP with memoized reconstruction. Time/Space O(n^2).
#include <bits/stdc++.h>
using namespace std;

string s;
vector<vector<string>> memo;
vector<vector<bool>> done;

string build(int i, int j) {
    if (i > j) return "";
    if (i == j) return string(1, s[i]);
    if (done[i][j]) return memo[i][j];
    done[i][j] = true;
    string res;
    if (s[i] == s[j]) {
        res = s[i] + build(i + 1, j - 1) + s[i];
    } else {
        string left = s[i] + build(i + 1, j) + s[i];
        string right = s[j] + build(i, j - 1) + s[j];
        if (left.size() != right.size())
            res = left.size() < right.size() ? left : right;
        else
            res = min(left, right);
    }
    return memo[i][j] = res;
}

string makePalindrome(const string& str) {
    s = str;
    int n = s.size();
    memo.assign(n, vector<string>(n));
    done.assign(n, vector<bool>(n, false));
    return build(0, n - 1);
}

int main() {
    cout << makePalindrome("race") << "\n";
    cout << makePalindrome("google") << "\n";
    return 0;
}
