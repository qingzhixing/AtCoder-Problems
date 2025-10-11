#include <iostream>
#include <deque>
using namespace std;

static const int MAX_N = 20;

static const int BLACK = -1, WHITE = 1, EMPTY = 0;

int n, m;
bool edge[MAX_N][MAX_N];
// -1 黑 1 白 0 未染色
int color[MAX_N];

int main()
{
	cin >> n >> m;
	for (int i = 1; i <= m; i++)
	{
		int u, v;
		cin >> u >> v;
		edge[u][v] = edge[v][u] = true;
	}

	// 染色
	// 循环处理非连通图情况
	int result = 0;
	for (int node = 1; node <= n; node++)
	{
		if (color[node] != EMPTY)
			continue;

		deque<int> handling_nodes;
		handling_nodes.push_back(node);
		color[node] = BLACK;

		while (handling_nodes.size())
		{
			int handling_node = handling_nodes.front();
			handling_nodes.pop_front();
			int next_color = color[handling_node] == BLACK ? WHITE : BLACK;
			for (int i = 1; i <= n; i++)
			{
				if (edge[handling_node][i])
				{
					if (color[i] == EMPTY)
					{
						handling_nodes.push_back(i);
						color[i] = next_color;
					}
					else if (color[i] != next_color)
					{
						result++;
						edge[handling_node][i] = edge[i][handling_node] = false;
					}
				}
			}
		}
	}

	cout << result;
	return 0;
}