#include <iostream>
#include <vector>
#include <queue>
#include <set>
using namespace std;

void Solution()
{
	int result = 0;

	int n;
	cin >> n;
	vector<int> r;
	set<int> value_set;

	for (int i = 0; i < n; i++)
	{
		int row;
		cin >> row;
		r.push_back(row);
		value_set.insert(row);
	}

	priority_queue<int, vector<int>, greater<int>> value_queue;
	for (auto value : value_set)
	{
		value_queue.push(value);
	}

	while (value_queue.size())
	{
		auto value = value_queue.top();
		value_queue.pop();

		for (int i = 0; i < n; i++)
		{
			if (r[i] != value)
				continue;
			if (i - 1 > 0 && r[i - 1] - r[i] > 1)
			{
				result += r[i - 1] - r[i] - 1;
				r[i - 1] = r[i] + 1;
				if (value_set.find(r[i] + 1) == value_set.end())
				{
					value_set.insert(r[i] + 1);
					value_queue.push(r[i] + 1);
				}
			}
			if (i + 1 < n && r[i + 1] - r[i] > 1)
			{
				result += r[i + 1] - r[i] - 1;
				r[i + 1] = r[i] + 1;
				if (value_set.find(r[i] + 1) == value_set.end())
				{
					value_set.insert(r[i] + 1);
					value_queue.push(r[i] + 1);
				}
			}
		}
	}
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