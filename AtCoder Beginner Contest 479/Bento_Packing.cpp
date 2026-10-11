#include <iostream>
using namespace std;

const int MAX_N = 21;
const int MAX_M = 1e9 + 10;

int n, m;
int v[MAX_N], w[MAX_N];

int main()
{
	cin >> n >> m;
	for (int i = 0; i < n; i++)
	{
		cin >> w[i] >> v[i];
	}

	long long result = 0;

	// 枚举所有可能的选择状态
	for (int state = 0; state < (1 << n); state++)
	{
		long long v_sum = 0;
		long long w_sum = 0;
		for (int idx = 0; idx < n; idx++)
		{
			// 当前物品被选
			if ((state >> idx) & 1)
			{
				v_sum += v[idx];
				w_sum += w[idx];
			}
		}
		if (v_sum > m)
		{
			continue;
		}
		result = max(result, w_sum);
	}

	cout << result << endl;
	return 0;
}