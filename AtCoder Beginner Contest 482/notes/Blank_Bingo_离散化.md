# C - Band Count

## 题目信息

题目链接：[atcoder.jp/contests/abc482/tasks/abc482_c](https://atcoder.jp/contests/abc482/tasks/abc482_c)

Clist Rating: 556

算法标签：离散化 模拟 前缀和

### 问题陈述

有一个由 $H$ 行和 $W$ 列组成的网格。 设 $(i,j)$ 表示从顶部算起第 $i$ 行、从左侧算起第 $j$ 列的单元格。
网格上放置了 $N$ 个方块，第 $i$ 个方块位于 $(R_i, C_i)$ 处。
需处理 $Q$ 次查询。 每个查询属于以下 $2$ 种类型之一。

- `1 u d` ：输出放置在第 $u$ 行至第 $d$ 行之间的方块数量。
- `2 l r` ：输出放置在第 $l$ 列至第 $r$ 列中的方块数量。

### 约束条件

- $1 \leq H, W \leq 10^9$
- $1 \leq N \leq \min(H \times W, 3 \times 10^5)$
- $1 \leq R_i \leq H$
- $1 \leq C_i \leq W$
- $(R_i, C_i) \neq (R_j, C_j)$ 若 $i \neq j$ 。
- $1 \leq Q \leq 3 \times 10^5$
- $1 \leq u \leq d \leq H$
- $1 \leq l \leq r \leq W$
- 所有输入值均为整数。

## 算法思路

对每个出现的坐标进行离散化，在此基础上求前缀和，即可实现区间查询。
