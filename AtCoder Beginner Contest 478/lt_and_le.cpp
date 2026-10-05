#include <iostream>
#include <vector>
#include <ranges>
#include <atcoder/scc>
using namespace std;

const int MAX_N = 2e5 + 10;

int n, q;

int main()
{
	cin >> n >> q;

	atcoder::scc_graph scc(n);

	// 储存严格小于号的边, from, to
	vector<pair<int, int>> strict_edge;

	while (q--)
	{
		int t, u, v;

		cin >> t >> u >> v;

		// to 0-indexed
		--u;
		--v;

		// 建立一条 u -> v 的边，表示 A[u] <= A[v];
		scc.add_edge(u, v);

		// 若 t = 1, 记录严格边
		if (t == 1)
		{
			strict_edge.push_back({u, v});
		}
	}

	// 获取拓扑排序后的强连通分量
	auto components = scc.scc();

	vector<int> value(n);

	// 计算每个点的值
	for (int idx = 0; idx < (int)components.size(); idx++)
	{
		for (int id : components[idx])
		{
			// 环内数字必然相等，每个点的值为其强连通分量的编号值
			value[id] = idx + 1;
		}
	}

	// 检测每个严格边是否满足小于关系
	for (const auto &[from, to] : strict_edge)
	{
		// 在同一个强连通分量里则不成立
		if (value[from] == value[to])
		{
			cout << "No" << endl;
			return 0;
		}
	}
	cout << "Yes" << endl;
	for (int i = 0; i < n; i++)
	{
		cout << value[i] << ' ';
	}
	cout << endl;
	return 0;
}