#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

const int MAX_N = 2e5 + 10;

int n, q;

// x, l, r
vector<tuple<int, int, int>> ops;

int s[MAX_N];

int main()
{
	cin >> n >> q;

	for (int i = 1; i <= q; i++)
	{
		int l, r, x;
		cin >> l >> r >> x;
		ops.push_back({x, l, r});
	}

	sort(ops.begin(), ops.end());

	// 对相同的 x 进行区间合并，然后再根据区间端点修改差分数组
	int last_num = 0;
	int last_l = 0;
	int last_r = 0;
	for (auto [num, l, r] : ops)
	{
		// 换数字了，结算上一个区间
		if (num != last_num)
		{
			s[last_l]++;
			s[last_r + 1]--;

			last_num = num;
			last_l = l;
			last_r = r;

			continue;
		}
		// 没换数字，判断能否合并区间
		if (l <= last_r)
		{
			// 能合并
			last_r = max(last_r, r);
			continue;
		}
		else
		{
			// 不能合并
			s[last_l]++;
			s[last_r + 1]--;

			last_l = l;
			last_r = r;
		}
	}
	// 结算最后一个区间
	s[last_l]++;
	s[last_r + 1]--;

	// 还原 s
	for (int i = 1; i <= n; i++)
	{
		s[i] += s[i - 1];
		cout << s[i] << ' ';
	}
	cout << endl;
	return 0;
}