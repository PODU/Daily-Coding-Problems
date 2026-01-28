// Day 974: Evaluate expression with parentheses, digits, +/- (no eval).
// Approach: single scan with a sign/result stack. Time O(n), Space O(n).
#include <bits/stdc++.h>
using namespace std;

long long evaluate(const string& s) {
    long long result = 0, num = 0;
    int sign = 1;
    vector<long long> st; // alternating: result, sign
    for (char c : s) {
        if (isdigit(c)) {
            num = num * 10 + (c - '0');
        } else if (c == '+') {
            result += sign * num; num = 0; sign = 1;
        } else if (c == '-') {
            result += sign * num; num = 0; sign = -1;
        } else if (c == '(') {
            st.push_back(result); st.push_back(sign);
            result = 0; sign = 1;
        } else if (c == ')') {
            result += sign * num; num = 0;
            long long prevSign = st.back(); st.pop_back();
            long long prevResult = st.back(); st.pop_back();
            result = prevResult + prevSign * result;
            sign = 1;
        }
        // spaces ignored
    }
    result += sign * num;
    return result;
}

int main() {
    cout << evaluate("-1 + (2 + 3)") << endl; // 4
    return 0;
}
