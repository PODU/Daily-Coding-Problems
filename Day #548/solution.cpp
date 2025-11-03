// Clock hand angle: minute=mm*6, hour=(hh%12)*30+mm*0.5, take abs diff, fold >180, round. O(1) time/space.
// Fun fact: the hands overlap (angle 0) 22 times a day, not 24.
#include <bits/stdc++.h>
using namespace std;

int clockAngle(const string& t) {
    int hh = stoi(t.substr(0, t.find(':')));
    int mm = stoi(t.substr(t.find(':') + 1));
    double minute = mm * 6.0;
    double hour = (hh % 12) * 30.0 + mm * 0.5;
    double diff = fabs(hour - minute);
    if (diff > 180.0) diff = 360.0 - diff;
    return (int)llround(diff);
}

int main() {
    cout << clockAngle("12:00") << "\n";
    cout << clockAngle("3:30") << "\n";
    cout << clockAngle("9:00") << "\n";
    return 0;
}
