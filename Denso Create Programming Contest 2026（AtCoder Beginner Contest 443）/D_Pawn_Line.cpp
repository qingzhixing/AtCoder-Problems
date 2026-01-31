#include <iostream>
#include <vector>
#include <deque>
using namespace std;

void Solution()
{
	int result = 0;

	int n;
	cin >> n;
	vector<int> r;

	int lowest = 4e5 + 114;
	for (int i = 0; i < n; i++)
	{
		int row;
		cin >> row;
		lowest = min(lowest, row);
		r.push_back(row);
	}

	// BFS
	deque<int> changing_idxs;
	vector<bool> changed_idxs(n + 10);

	// 查找所有最小值为lowest的节点依次入队作为BFS起点
	for (int i = 0; i < n; i++)
	{
		if (r[i] == lowest)
		{
			changed_idxs[i] = true;
			changing_idxs.push_back(i);
		}
	}

	while (!changing_idxs.empty())
	{
		auto current_idx = changing_idxs.front();
		changing_idxs.pop_front();
		// 激活相邻index
		int left_idx = current_idx - 1, right_idx = current_idx + 1;
		if (left_idx >= 0 && !changed_idxs[left_idx])
		{
			changed_idxs[left_idx] = true;
			changing_idxs.push_back(left_idx);

			// 移动节点
			int delta = r[left_idx] - r[current_idx];
			if (delta > 1)
			{
				// 移动到差一格就好
				r[left_idx] = r[current_idx] + 1;
				result += delta - 1;
			}
		}
		if (right_idx < n && !changed_idxs[right_idx])
		{
			changed_idxs[right_idx] = true;
			changing_idxs.push_back(right_idx);

			// 移动节点
			int delta = r[right_idx] - r[current_idx];

			if (delta > 1)
			{
				// 移动到差一格就好
				r[right_idx] = r[current_idx] + 1;
				result += delta - 1;
			}
		}
	}
	
	for (int i = 0; i < n; i++)
	{
		cout << r[i] << ' ';
	}
	cout << endl;

	cout << result << endl;
}

int main()
{
	int t;
	cin >> t;
	while (t--)
	{
		Solution();
	}
	return 0;
}