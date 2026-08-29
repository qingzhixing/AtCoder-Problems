#include <iostream>
#include <vector>
using namespace std;

int main()
{
	int n, k;
	cin >> n >> k;

	vector<int> path(n);
	[n, k](this auto self, int index, int sum, vector<int> &path)
	{
		// 正在决定最后一个数，直接计算
		if (index == n - 1)
		{
			// 可以填则填，否则跳过
			if ((k - sum) % n != 0)
			{
				return;
			}
			// 填入数字并输出解
			path[index] = (k - sum) / n;

			for (auto num : path)
			{
				cout << num << ' ';
			}
			puts("");
			return;
		}

		// 不是最后一个数字则枚举
		int max_num = (k - sum) / (index + 1);
		for (int num = 0; num <= max_num; num++)
		{
			path[index] = num;
			self(index + 1, sum + num * (index + 1), path);
		}
	}(0, 0, path);
	return 0;
}