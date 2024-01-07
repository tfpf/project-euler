#! /usr/bin/env python3

import itertools
import math


N = 18
B = math.ceil(N / 60)
sieve = [False] * (N + 1)
print(f'{N=} {B=}')


def algorithm_31(n, delta):
    for (x, y, k) in algorithm_41(delta):
        sieve[n] = not sieve[n]

def algorithm_41(delta):
    match delta:
        case 1: (f, g) = (15, 1)
        case 13: (f, g) = (14, 3)
        case 17: (f, g) = (13, 1)
        case 29: (f, g) = (14, 5)
        case 37: (f, g) = (13, 9)
        case 41: (f, g) = (13, 5)
        case 49: (f, g) = (15, 7)
        case 53: (f, g) = (14, 7)
    (x, y0, k0) = (f, g, (4 * f ** 2 + g ** 2 - delta) // 60)
    while k0 < B:
        (k0, x) = (k0 + 2 * x + 15, x + 15)
    print(x, y0, k0)
    while True:
        (x, k0) = (x - 15, k0 - 2 * x - 15)
        print(x, y0, k0)
        if x <= 0:
            return
        while k0 < 0:
            (k0, y0) = (k0 + y0 + 15, y0 + 30)
        (k, y) = (k0, y0)
        while k < B:
            yield (x, y, k)
            (k, y) = (k + y + 15, y + 30)


for n in range(N + 1):
    delta = n % 60
    if delta in {1, 13, 17, 29, 37, 41, 49, 53}:
        algorithm_31(n, delta)

for item in enumerate(sieve):
    print(item)


# for (f, g) in itertools.product(range(1, 16), reversed(range(1, 31))):
#     if (4 * f ** 2 + g ** 2) % 60 == 53:
#         print(f, g)
