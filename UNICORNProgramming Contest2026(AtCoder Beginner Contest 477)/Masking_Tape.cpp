#include <iostream>
#include <set>
#include <map>
using namespace std;

const int MAX_N = 3e5 + 10;

int n, q;
char global_color;

// 记录保留格子的颜色
map<int, char> remain;

// 记录要删除保留的坐标
set<int> deleting;

int main()
{
	cin >> n >> q;
	global_color = 'a';
	while (q--)
	{
		int op;
		cin >> op;
		// place / remove
		if (op == 1)
		{
			int number;
			cin >> number;

			// 若 number 在 deleting 中，删除deleting 标记
			if (deleting.contains(number))
			{
				deleting.erase(number);
				continue;
			}

			// 若 number 在 remain 中, 打上 deleting 标记
			if (remain.contains(number))
			{
				deleting.insert(number);
				continue;
			}

			// 若 number 不在 remain 中, 将其加入 remain
			if (!remain.contains(number))
			{
				remain[number] = global_color;
			}
		}
		// change color
		else
		{
			char ch;
			cin >> ch;

			// 删除带有标记的保留格
			for (auto key : deleting)
			{
				remain.erase(key);
			}

			// 清空标记
			deleting.clear();

			// 更新全局颜色
			global_color = ch;
		}
	}

	for (int i = 1; i <= n; i++)
	{
		if (remain.contains(i))
		{
			cout << remain[i];
		}
		else
		{
			cout << global_color;
		}
	}
	cout << endl;

	return 0;
}