#include <iostream>
using namespace std;

const int MAX_N = 8010;

int n, s;
long long a[MAX_N];
long long prefix[MAX_N];
long long l;

int main()
{
	cin >> n >> s >> l;
	for (int i = 1; i <= n - 1; i++)
	{
		cin >> a[i];
		prefix[i] = prefix[i - 1] + a[i];
	}

	int result = 0;
	// 枚举走法，先往左走 i 格，再往右走 j 格
	// 最多从 s 走到 0
	for (int i = 0; i <= s - 1; i++)
	{
		for (int j = 0; j <= n - s + i; j++)
		{
			// 从 s 走到 s - i, 经过了 a[s - i] ~ a[s - 1]
			long long coast_left = prefix[s - 1] - prefix[s - i - 1];
			// 从 s - i 走到了 s - i + j, 经过了 a[s - i] ~ a[s - i + j - 1]
			long long coast_right = prefix[s - i + j - 1] - prefix[s - i - 1];

			if (coast_left + coast_right <= l)
			{
				result = max(result, 1 + i + max(0, j - i));
			}
		}
	}
	// 枚举走法，先往右走 i 格，再往左走 j 格
	// 最多从 s 走到 n
	for (int i = 0; i <= n - s; i++)
	{
		for (int j = 0; j <= s + i - 1; j++)
		{
			// 从 s 走到了 s + i, 经过了 a[s] ~ a[s + i - 1]
			long long coast_right = prefix[s + i - 1] - prefix[s - 1];
			// 从 s + i  走到了 s + i - j, 经过了 a[s + i - j] ~ a[s + i - 1]
			long long coast_left = prefix[s + i - 1] - prefix[s + i - j - 1];

			if (coast_left + coast_right <= l)
			{
				result = max(result, 1 + i + max(0, j - i));
			}
		}
	}

	cout << result << endl;
	return 0;
}