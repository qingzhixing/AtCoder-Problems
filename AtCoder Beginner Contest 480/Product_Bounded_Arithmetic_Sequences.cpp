#include <iostream>
using namespace std;

const long long MAX_N = 1e18;

int main()
{
	long long n;
	cin >> n;

	long long result = 0;

	// start: 数列第一项的值; len: 数列的长度
	const auto get_valid_diff_cnt = [&](long long start, long long len)
	{
		long long l = 0, r = 1e9;
		while (l < r)
		{
			const auto mid = (l + r + 1) >> 1;

			long long quotient = n;
			for (long long i = 0; i < len; i++)
			{
				quotient /= (start + i * mid);
			}

			if (quotient > 0)
			{
				// n >= product of array
				l = mid;
			}
			else
			{
				r = mid - 1;
			}
		}
		return l;
	};

	for (long long start = 1; start <= n / 3; start++)
	{
		long long quotient = n / (start) / (start + 1) / (start + 2);
		// 最小数列都不满足，后面的也不可能满足了
		if (quotient <= 0)
		{
			break;
		}
		long long len = 3;
		// 当前 len 至少存在一个解
		while (quotient > 0)
		{
			result += get_valid_diff_cnt(start, len);
			len++;
			// 除去新产生的项
			quotient /= (start + len - 1);
		}
	}

	cout << result << endl;

	return 0;
}