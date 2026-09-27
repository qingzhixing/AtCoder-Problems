#include <iostream>
#include <cstring>
#include <queue>
using namespace std;

const int MAX_N = 2e5 + 10;

int n, q;
long long a[MAX_N], b[MAX_N];

// 每一个点到 N + 1 的最短路径
long long dist[MAX_N];

// a 的前缀和数组
long long p[MAX_N];

// 建图，用于 Dijkstra
// to, weight
vector<pair<int, long long>> edges[MAX_N];

void init_dist()
{
	memset(dist, 0x3f, sizeof(dist));
	priority_queue<pair<long long, int>, vector<pair<long long, int>>, greater<>> q;
	dist[n + 1] = 0;
	q.push({0, n + 1});
	while (q.size())
	{
		auto [d, id] = q.top();
		q.pop();

		// 过期数据跳过
		if (d != dist[id])
		{
			continue;
		}

		// 利用当前点松弛其他点
		for (auto [to, weight] : edges[id])
		{
			if (dist[to] > dist[id] + weight)
			{
				dist[to] = dist[id] + weight;
				q.push({dist[to], to});
			}
		}
	}
}

int main()
{
	cin >> n >> q;
	for (int i = 1; i <= n; i++)
	{
		cin >> a[i];
		p[i] = p[i - 1] + a[i];

		edges[i].push_back({(i % n) + 1, a[i]});
		edges[(i % n) + 1].push_back({i, a[i]});
	}
	for (int i = 1; i <= n; i++)
	{
		cin >> b[i];

		edges[i].push_back({n + 1, b[i]});
		edges[n + 1].push_back({i, b[i]});
	}

	init_dist();

	while (q--)
	{
		int s, t;
		cin >> s >> t;

		if (t == n + 1)
		{
			cout << dist[s] << endl;
			continue;
		}

		// 仅走外圈，顺时针
		auto out_clockwise = abs(p[t - 1] - p[s - 1]);
		// 走外圈，逆时针
		auto out_anticlockwise = p[n] - out_clockwise;
		// 过中心
		auto pass_center = dist[s] + dist[t];

		cout << min(pass_center, min(out_clockwise, out_anticlockwise)) << endl;
	}
	return 0;
}