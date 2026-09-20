#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

const int MAX_N = 2e5 + 10;

int n, m, k;
long long x, y;

int a[MAX_N];
int b[MAX_N];
// 买 b[i] 需要多少张 K 元纸币
int b_bill[MAX_N];
long long pre_a[MAX_N];
long long pre_b[MAX_N];
long long pre_b_bill[MAX_N];

// 找到当前金额最多能买前多少个a
int find(long long money)
{
	int l = 0, r = n;
	while (l < r)
	{
		int mid = (l + r + 1) >> 1;
		// 满足条件
		if (pre_a[mid] <= money)
		{
			l = mid;
		}
		else
		{
			r = mid - 1;
		}
	}
	return l;
}

int main()
{
	cin >> n >> m >> k;
	cin >> x >> y;
	for (int i = 1; i <= n; i++)
	{
		cin >> a[i];
	}

	for (int i = 1; i <= m; i++)
	{
		cin >> b[i];
	}

	sort(a + 1, a + 1 + n);
	sort(b + 1, b + 1 + m);

	for (int i = 1; i <= m; i++)
	{
		b_bill[i] = (b[i] + k - 1) / k;
	}

	// 求前缀和
	for (int i = 1; i <= n; i++)
	{
		pre_a[i] = pre_a[i - 1] + a[i];
	}
	for (int i = 1; i <= m; i++)
	{
		pre_b[i] = pre_b[i - 1] + b[i];
		pre_b_bill[i] = pre_b_bill[i - 1] + b_bill[i];
	}

	int result = 0;
	// 枚举 b 买了多少个
	for (int take_b = 0; take_b <= m; take_b++)
	{
		auto money_left = x + y * k - pre_b[take_b];

		auto bill_left = y - pre_b_bill[take_b];

		// 钱不够
		if (money_left < 0 || bill_left < 0)
		{
			break;
		}

		auto take_a = find(money_left);

		result = max(result, take_a + take_b);
	}
	cout << result << endl;
	return 0;
}