# Loops 2
We use the integers *a*, *b* and *n* to create the following series:<br><br>
(a + $2^0$ * b), (a + $2^0$ * b + $2^1$ * b), ..., (a + $2^0$ * b + $2^1$ * b + ... + 2<sup>n-1</sup> * b)
<br><br>
You are given *q* queries in the form of *a*, *b* and *n*. For each query, print the series corresponding to the given *a*, *b* and *n* values as a single line of *n* space-separated integers.

**Input Format**<br>
The first line contains an integer *q* denoting the number of queries.
Each line *i* of the *q* subsequent lines contains three space-separated integers describing the respective *a<sub>i</sub>*, *b<sub>i</sub>* and *n<sub>i</sub>* values for that query.

**Constraints**<br>
- 0 ≤ *q* ≤ 500
- 0 ≤ *a*, *b* ≤ 50
- 1 ≤ *n* ≤ 15

**Output Format**<br>
For each query, print the corresponding series on a new line. Each series must be printed in order as a single line of *n* space-separated integers.

**Sample Input**<br>
```
2
0 2 10
5 3 5
```

**Sample Output**<br>
```
2 6 14 30 62 126 254 510 1022 2046
8 14 26 50 98
```