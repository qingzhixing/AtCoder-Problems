#include <iostream>
#include <vector>
using namespace std;

const int MAX_N = 1e5 + 10;
const int MAX_M = 1e8;

int n, q;

// 找到一个最大的 x 使得 x(x + 1) / 2 <= k
long long max_root(long long k)
{
	long long l = 0, r = n + 1;
	while (l < r)
	{
		auto mid = (l + r + 1) >> 1;
		if (mid * (mid + 1) / 2 <= k)
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
	cin >> n >> q;
	vector<int> a(n + 10);
	// a 前缀和
	vector<long long> p(n + 10);
	// p 前缀和
	vector<long long> pp(n + 10);
	for (int i = 1; i <= n; i++)
	{
		cin >> a[i];
		p[i] = p[i - 1] + a[i];
		pp[i] = pp[i - 1] + p[i];
	}

	while (q--)
	{
		long long L, R;
		cin >> L >> R;
		L -= 1;
		const auto root_L = max_root(L);
		const auto root_R = max_root(R);
		const auto pb_L = pp[root_L] + p[L - root_L * (root_L + 1) / 2];
		const auto pb_R = pp[root_R] + p[R - root_R * (root_R + 1) / 2];
		cout << pb_R - pb_L << endl;
	}
	return 0;
}