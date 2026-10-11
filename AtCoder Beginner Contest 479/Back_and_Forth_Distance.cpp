#include <iostream>
#include <vector>
using namespace std;

const int MAX_N = 3e5 + 10;
const int MAX_M = 1e9;
const int INF = MAX_M + 10;

int main()
{
	int n, m;
	cin >> n >> m;

	vector<int> L(n + 10, INF);
	vector<int> R(n + 10, INF);

	for (int i = 2; i <= n; i++)
	{
		cin >> L[i];
	}

	for (int i = 1; i <= n - 1; i++)
	{
		cin >> R[i];
	}

	// 前缀向后最小值
	vector<int> prefix_min_R(n + 10, INF);
	// 后缀向前最小值
	vector<int> suffix_min_L(n + 10, INF);

	for (int i = 1; i <= n; i++)
	{
		// printf("L[%d] = %d\n", i, L[i]);
		prefix_min_R[i] = min(prefix_min_R[i - 1], R[i]);
	}
	for (int i = n; i >= 1; i--)
	{
		// printf("R[%d] = %d\n", i, R[i]);
		suffix_min_L[i] = min(suffix_min_L[i + 1], L[i]);
	}

	while (m--)
	{
		int s, t;
		cin >> s >> t;
		if (s == t)
		{
			cout << 0 << endl;
			continue;
		}
		if (s == 1)
		{
			cout << R[s] << endl;
			continue;
		}
		if (s == n)
		{
			cout << L[s] << endl;
			continue;
		}

		// 向右跳则尝试向左跳到向右最小点再向右跳或者直接跳
		if (t > s)
		{
			cout << min(R[s], L[s] + prefix_min_R[s - 1]) << endl;
		}
		else
		{
			// 向左跳则尝试向右跳到向左最小点再向左跳或者直接跳
			cout << min(L[s], R[s] + suffix_min_L[s + 1]) << endl;
		}
	}
	return 0;
}