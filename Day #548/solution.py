# Day 548: Clock hand angle: minute=mm*6, hour=(hh%12)*30+mm*0.5, take abs diff, fold >180, round. O(1) time/space.
# Fun fact: the hands overlap (angle 0) 22 times a day, not 24.

def clock_angle(t):
    hh, mm = (int(x) for x in t.split(':'))
    minute = mm * 6.0
    hour = (hh % 12) * 30.0 + mm * 0.5
    diff = abs(hour - minute)
    if diff > 180.0:
        diff = 360.0 - diff
    return round(diff)


if __name__ == "__main__":
    print(clock_angle("12:00"))
    print(clock_angle("3:30"))
    print(clock_angle("9:00"))
