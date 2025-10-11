#include <iostream>
using namespace std;

static const int MAX_N = 110;

int f_pre_sum[MAX_N];
int n;

int f(int val)
{
	int result = 0;
	while (val)
	{
		result += val % 10;
		val /= 10;
	}
	return result;
}

int main()
{
	cin >> n;
	f_pre_sum[0] = 1;
	for (int i = 1; i <= n; i++)
	{
		int current_val = f_pre_sum[i - 1];
		f_pre_sum[i] = f_pre_sum[i - 1] + f(current_val);
		if (i == n)
		{
			cout << current_val << endl;
			break;
		}
	}
	return 0;
}