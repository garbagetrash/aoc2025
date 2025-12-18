#!/usr/bin/env python3
import numpy as np
import scipy.optimize


def parse(filename):
    cntr = 0
    with open(filename, "r") as fid:
        data = fid.read()
        output = []
        for line in data.splitlines():
            cntr += 1
            toks = line.split(' ')
            cstr = toks[-1]
            bstr = toks[1:-1]
            buttons = []
            for b in bstr:
                b = b.replace('(', '')
                b = b.replace(')', '')
                buttons.append([int(x) for x in b.split(',')])
            cstr = cstr.replace('{', '')
            cstr = cstr.replace('}', '')
            counters = [int(x) for x in cstr.split(',')]
            output.append((buttons, counters))
        return output

def btn_idxs_to_vec(btn_idxs, m):
    output = np.zeros(m)
    for i in btn_idxs:
        output[i] = 1
    return output

def create_axb(problem):
    buttons, counters = problem
    n = len(buttons)
    m = len(counters)
    A = np.zeros((m+1, n))
    b = np.zeros(m+1)
    b[:m] = counters

    for i in range(n):
        A[:m, i] = btn_idxs_to_vec(buttons[i], m)
    A[m, :] = 1

    return A, b

def solve(A, b):
    b = b.T
    start = int(np.max(b))
    end = int(np.sum(b)) + 1
    for n in range(start, end):
        b[-1] = n
        c = np.ones(A.shape[1])
        result = scipy.optimize.linprog(c, A_eq=A, b_eq=b, method='highs', integrality=1)
        if result['success']:
            print(f"n: {n}, {result['x']}")
            # Verify
            assert np.sum(np.abs(A @ result['x'] - b)) < 1e-3
            return n
    raise RuntimeError("failed")


if __name__ == "__main__":
    problems = parse("/Users/voyager/git/aoc/aoc2025/inputs/day10.txt")
    answer = 0
    #print(f"# Problems: {len(problems)}")
    for i, problem in enumerate(problems):
        A, b = create_axb(problem)
        #print(i, problem, A, b)
        answer += solve(A, b)
        #print(answer)
    print(f"Solution: {answer}")
