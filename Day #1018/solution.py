# Day 1018: Clock angle: O(1) time, O(1) space.
# minute=mm*6, hour=(hh%12)*30+mm*0.5, diff=|h-m|, angle=min(diff,360-diff), rounded.
def clock_angle(hh, mm):
    minute = mm * 6.0
    hour = (hh % 12) * 30.0 + mm * 0.5
    diff = abs(hour - minute)
    angle = min(diff, 360.0 - diff)
    return round(angle)


if __name__ == "__main__":
    print(f"{3:02d}:{30:02d} -> {clock_angle(3, 30)}")
    print(f"{12:02d}:{0:02d} -> {clock_angle(12, 0)}")
