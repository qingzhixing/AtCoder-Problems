#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

const int MAX_N = 2e5 + 10;
const int MAX_M = 1e9;
const int INF = 1e9 + 10;

int main()
{
	int n, k, t;
	cin >> n >> k >> t;
	vector<pair<int, int>> ranges(n);
	for (auto &range : ranges)
	{
		int a;
		cin >> a;
		range = {a, a + k};
	}

	sort(ranges.begin(), ranges.end());

	// for (const auto &range : ranges)
	// {
	// 	cout << range.first << ' ' << range.second << endl;
	// }

	vector<pair<int, int>> combined_ranges;
	pair<int, int> current_range = ranges[0];
	for (const auto &range : ranges)
	{
		// 间断，结算上一个区间
		if (range.first > current_range.second)
		{
			combined_ranges.push_back(current_range);
			current_range = range;
			continue;
		}

		current_range.second = range.second;
	}
	// 结算最后一个区间
	combined_ranges.push_back(current_range);

	// for (const auto &range : combined_ranges)
	// {
	// 	cout << range.first << ' ' << range.second << endl;
	// }

	int current_point = 0;
	int result = 0;
	for (const auto &range : combined_ranges)
	{
		// cout << "Current: " << current_point << " Step: " << result << endl;

		// 包含了该点
		if (range.first <= current_point)
		{
			current_point = range.second;
			continue;
		}

		if (current_point >= t)
		{
			break;
		}

		// 否则走到该区间左端点或者终点
		int target = min(range.first, t);
		int step = (target + k - 1 - current_point) / k;
		result += step;
		current_point += k * step;
		current_point = max(current_point, range.second);
	}

	// 看看有没有走到终点
	if (current_point < t)
	{
		int step = (t + k - 1 - current_point) / k;
		result += step;
		current_point += k * step;
	}
	cout << result << endl;
	return 0;
}