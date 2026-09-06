#include <iostream>
#include <algorithm>
#include <cstring>
#include <climits>
using namespace std;

const int MAX_N = 2e5 + 10;

int n;

// 优惠金额
int discount[MAX_N] = {};

void Solution()
{

	cin >> n;

	// 单价最低商品
	int low_a = 1e9 + 10;

	// 所有物品原价购买
	long long sum_a = 0;

	for (int i = 1; i <= n; i++)
	{
		int a, b;
		cin >> a >> b;

		low_a = min(low_a, a);

		sum_a += a;
		discount[i] = a - b;
	}

	// 从大到小进行排序
	sort(discount + 1, discount + 1 + n, greater<int>());

	// 枚举不同优惠券的使用情况
	long long result = LLONG_MAX;

	// 前 i 个物品使用优惠券
	// 前 i 个物品的总折扣
	long long prefix = 0;
	for (int i = 0; i <= n; i++)
	{
		prefix += discount[i];
		int coupon_need = i - (n - i);
		long long cost = sum_a - prefix + 1LL * max(0, coupon_need) * low_a;
		result = min(result, cost);
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