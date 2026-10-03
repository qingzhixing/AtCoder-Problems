#include <iostream>
using namespace std;

#error TODO:Wrong Answer.

const int MAX_N = 2e5 + 10;

int n, q;

int parent[MAX_N];
int dist[MAX_N];

int find_root(int id)
{
	if (parent[id] == id)
	{
		return id;
	}

	int p = parent[id];
	int root = find_root(p);

	dist[id] += dist[p];
	parent[id] = root;

	return root;
}

int main()
{
	cin >> n >> q;

	// 初始化并查集
	for (int i = 1; i <= n; i++)
	{
		parent[i] = i;
		dist[i] = 0;
	}

	bool wrong_answer = false;
	while (q--)
	{
		int t, u, v;
		cin >> t >> u >> v;

		// 答案已知错误则跳过
		if (wrong_answer)
		{
			continue;
		}

		int root_u = find_root(u);
		int root_v = find_root(v);

		// 连接 v -> u = t

		// 合并两个集合
		if (root_u != root_v)
		{
			int delta = dist[u] + t - dist[v];

			// 保证 dist 始终非负
			if (delta >= 0)
			{
				parent[root_v] = root_u;
				dist[root_v] = delta;
			}
			else
			{
				parent[root_u] = root_v;
				dist[root_u] = -delta;
			}
		}
		else
		{
			// 检验答案是否正确
			if (!((t && dist[u] < dist[v]) || (!t && dist[u] <= dist[v])))
			{
				// printf("Wrong answer judging t=%d, u=%d, v=%d\n", t, u, v);
				// printf("dist[%d] = %d, dist[%d] = %d\n", u, dist[u], v, dist[v]);
				wrong_answer = true;
			}
		}
	}
	if (wrong_answer)
	{
		cout << "No" << endl;
		return 0;
	}

	// 检查每个点到 root 的距离
	for (int i = 1; i <= n; i++)
	{
		find_root(i);
		// printf("dist[%d]=%d\n", i, dist[i]);
		if (dist[i] >= n)
		{
			cout << "No" << endl;
			return 0;
		}
	}

	// 输出每个点的距离
	cout << "Yes" << endl;
	for (int i = 1; i <= n; i++)
	{
		cout << dist[i] + 1 << ' ';
	}
	cout << endl;
	return 0;
}