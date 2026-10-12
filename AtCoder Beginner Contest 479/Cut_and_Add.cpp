#include <iostream>
#include <vector>
#include <queue>
#include <algorithm>
using namespace std;

const int MAX_N = 2e5 + 10;
const int MAX_M = MAX_N;

int main()
{
	int n;
	cin >> n;

	// edge[from] = {to, edge_id}
	vector<vector<pair<int, int>>> edges(n);
	for (int edge_id = 1; edge_id <= n - 1; edge_id++)
	{
		int u, v;
		cin >> u >> v;

		// 0 - indexed
		--u;
		--v;

		edges[u].push_back({v, edge_id});
		edges[v].push_back({u, edge_id});
	}

	// 求 BFS序 以及初始化 父节点 与 边信息
	vector<int> BFS_order;
	vector<int> parent(n, -1);
	// 当前节点连接向父节点的那条边
	vector<int> parent_edge_id(n);

	queue<int> q;
	// 根节点特殊父亲
	parent[0] = -2;
	q.push(0);
	while (q.size())
	{
		auto from = q.front();
		q.pop();
		BFS_order.push_back(from);

		for (const auto &[to, edge_id] : edges[from])
		{
			// 父节点未初始化，即为 from 的子节点
			if (parent[to] == -1)
			{
				parent[to] = from;
				parent_edge_id[to] = edge_id;
				q.push(to);
			}
		}
	}

	// 求子树大小
	long long global_add = 0;
	// 延迟标记
	vector<long long> lazy(n);
	// 子树大小
	vector<int> subtree_size(n, 1);

	// 逆转 BFS序, 从叶子节点向根节点求
	reverse(BFS_order.begin(), BFS_order.end());
	for (const auto &from : BFS_order)
	{
		// 求当前节点的子树大小
		for (const auto &[to, edge_id] : edges[from])
		{
			// 如果是父节点则跳过
			if (to == parent[from])
			{
				continue;
			}
			subtree_size[from] += subtree_size[to];
		}

		// 当前被删除的边
		const auto deleted_edge_id = parent_edge_id[from];
		if (2 * subtree_size[from] < n)
		{
			// 补集更大
			global_add += deleted_edge_id;
			lazy[from] -= deleted_edge_id;
		}
		else
		{
			// 子树更大
			lazy[from] += deleted_edge_id;
		}
	}

	// 传播 lazy
	// 恢复 BFS序，从根节点往叶子节点传递
	reverse(BFS_order.begin(), BFS_order.end());
	for (const auto &from : BFS_order)
	{
		for (const auto &[to, edge_id] : edges[from])
		{
			// 如果是父节点则跳过
			if (to == parent[from])
			{
				continue;
			}
			lazy[to] += lazy[from];
		}
	}

	// 输出
	for (int i = 0; i < n; i++)
	{
		cout << global_add + lazy[i] << ' ';
	}
	cout << endl;

	return 0;
}