# Day 1252: Clock angle: minute=mm*6, hour=(hh%12)*30+mm*0.5, take min(diff,360-diff), round. O(1) time/space.
import math


def clock_angle(hh: int, mm: int) -> int:
    minute = mm * 6.0
    hour = (hh % 12) * 30.0 + mm * 0.5
    diff = abs(hour - minute)
    diff = min(diff, 360.0 - diff)
    return math.floor(diff + 0.5)


if __name__ == "__main__":
    print(clock_angle(12, 30))
    print(clock_angle(3, 15))
