#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

#error WA

const int MAX_N = 2e5 + 10;

int n, m, k;
long long x, y;

// price, [0, 1]: 0 -> B, 1 -> A
vector<pair<int, int>> prices;

int main()
{
	cin >> n >> m >> k;
	cin >> x >> y;
	for (int i = 1; i <= n; i++)
	{
		int a;
		cin >> a;
		prices.push_back({a, 1});
	}
	for (int i = 1; i <= m; i++)
	{
		int b;
		cin >> b;
		prices.push_back({b, 0});
	}

	sort(prices.begin(), prices.end());

	int result = 0;
	for (auto [price, from] : prices)
	{
		if (from == 0)
		{
			auto need_y = (price + k - 1) / k;
			if (need_y > y)
			{
				continue;
			}
			y -= need_y;
			x += k - (price % k);
			result++;
		}
		else
		{
			auto need_y = (max(0LL, price - x) + k - 1) / k;
			if (price > x && need_y > y)
			{
				continue;
			}
			x = max(0LL, x - price);
			y -= need_y;
			x += k - (price % k);
			result++;
		}
	}

	cout << result << endl;
	return 0;
}