#include <iostream>
using namespace std;

const int MAX_N = 2e5 + 10;

int n, k;
int a[MAX_N], p[MAX_N];
bool filtered_a_val[MAX_N];
bool filtered_p_val[MAX_N];
bool filtered_p_id[MAX_N];

int main()
{
	cin >> n >> k;
	for (int i = 1; i <= n; i++)
	{
		cin >> a[i];
	}
	for (int i = 1; i <= k; i++)
	{
		cin >> p[i];
		filtered_a_val[p[i]] = true;
	}
	// 从后往前设置 filtered_p, 因为后方优先度高
	for (int i = k; i >= 1; i--)
	{
		if (filtered_p_val[p[i]])
		{
			filtered_p_id[i] = true;
			continue;
		}
		filtered_p_val[p[i]] = true;
	}

	// 输出 a 中剩余元素
	for (int i = 1; i <= n; i++)
	{
		if (!filtered_a_val[a[i]])
		{
			cout << a[i] << ' ';
		}
	}

	// 输出 p 中剩余元素
	for (int i = 1; i <= k; i++)
	{
		if (!filtered_p_id[i])
		{
			cout << p[i] << ' ';
		}
	}

	cout << endl;
	return 0;
}