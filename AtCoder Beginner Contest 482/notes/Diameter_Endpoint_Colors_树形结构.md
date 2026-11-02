# E - Diameter Endpoint Colors

## 题目信息

题目链接：[atcoder.jp/contests/abc482/tasks/abc482_e](https://atcoder.jp/contests/abc482/tasks/abc482_e)

Clist Rating: 1503

算法标签：图论 模拟 树形结构 组合计数

## 题目大意

给定一棵树，所有节点被染色，求点对(u, v)数量，满足 u < v, dist(u, v) = 树的直径，并且他们颜色相等。

## 算法思路

- 这是一道比较典型的模板题，考察对树的直径，树的中心的求解。可惜我没见过。
- 为了之后求树的中心，中心唯一化，需要确保直径长度为偶数。我们可以在每条边中间插入一个虚拟节点，来实现这一点。
- 不难发现，虚拟节点的颜色对题目的解并无影响，因为虚拟节点不可能是叶子节点，所以不可能作为构成直径的端点。

### Step.1 求直径

我们可以使用两次 bfs / dfs 求解树的直径。

- 第一次，从某个点出发做搜索，求出距离当前点最远的节点。
- 第二次，从那个最远的节点出发，找到距离其最远的点。

这两个点之间的距离就是树的直径。

代码如下：

```Rust
// 从 root 开始进行 bfs, 求各点到 root 的距离
let bfs_dist = |root: usize| -> Vec<usize> {
    let mut dist = vec![n; n];
    dist[root] = 0;

    let mut q = VecDeque::from([root]);
    while let Some(current) = q.pop_front() {
        for &nxt in &adj[current] {
            if dist[nxt] > dist[current] + 1 {
                dist[nxt] = dist[current] + 1;
                q.push_back(nxt);
            }
        }
    }
    dist
};

// dist 当中最远的那个点
let furthest_id =
    |dist: &Vec<usize>| dist.iter().enumerate().max_by_key(|&(_, &v)| v).unwrap().0;

// 求数的直径
let dist0 = bfs_dist(0);
// 直径端点之一
let v1 = furthest_id(&dist0);
// 所有点到 v1 的距离
let dist1 = bfs_dist(v1);
// 直径端点之一
let v2 = furthest_id(&dist1);
// 所有点到 v2 的距离
let dist2 = bfs_dist(v2);

// 树的直径
let diam = dist2[v1];
// 由于插入虚拟节点，树的直径长度一定为偶数
assert!(diam % 2 == 0);
```

### Step.2 求树的中心

- 树的中心为：在直径路径上，距离直径端点距离相等的点。若直径长度为偶数，中心只有一个，为奇数则有两个。
- 我们可以枚举每个点到直径端点的距离，相等则为中心。

```Rust
// 寻找树的中心
let mut center = n + 1;
for i in 0..n {
    // 到直径端点距离相等
    if dist1[i] == diam / 2 && dist2[i] == diam / 2 {
        center = i;
    }
}
let center = center;
// 每个点到 center 的距离
let dist = bfs_dist(center);
```

### Step.3 DFS 统计答案

- 定义 `freq[colors]`为其他分支内距离中心点距离为直径 / 2 的同颜色的点的数量。
- 从中心出发，枚举每个分支，找到距离中心，dist = 直径 / 2 的节点。每个节点可以贡献它颜色的 `freq[color]` 个答案。
- 遍历完一个分支之后再把当前分支的满足条件的点加入 `freq[]`。这样能保证同一个分支的点不能互相匹配。

```Rust
// 当前已遍历的其他分支离中心点距离为 diam / 2 的点的颜色频率
let mut color_total_freq = vec![0usize; n];

// 从 current 往后继续遍历，统计答案
for &branch in &adj[center] {
    // 当前分支中符合要求的点
    let mut valid_points: Vec<usize> = Vec::new();

    // 当前迭代栈
    let mut stack = vec![(branch, center)];
    while let Some((current, parent)) = stack.pop() {
        if dist[current] == diam / 2 {
            valid_points.push(current);
        }

        // 往后扩展
        for &nxt in &adj[current] {
            if nxt == parent {
                continue;
            }
            stack.push((nxt, current));
        }
    }

    // 结算点
    for &point in &valid_points {
        let color = colors[point];
        // 答案增加 其他分支上颜色相同的点的频率
        result += color_total_freq[color];
    }

    // 将当前分支的点加入统计
    for &point in &valid_points {
        let color = colors[point];
        color_total_freq[color] += 1;
    }
}
```

## 复杂度

时间复杂度：$O(n)$

空间复杂度：$O(n)$
