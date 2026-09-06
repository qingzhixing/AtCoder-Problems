#include <iostream>
using namespace std;

const int MAX_N = 1e5 + 10;
const int MAX_M = 1e9 + 10;
const long long INF = 11451419198100000L;

int n;
int a[MAX_N];
int b[MAX_N];

int main()
{
	int n;
	cin >> n;
	for (int i = 1; i <= n; i++)
	{
		cin >> a[i];
	}

	bool no_ans = true;

	for (int i = 1; i <= n; i++)
	{
		cin >> b[i];

		// 至少找到一个 a[i] > b[i]
		if (a[i] > b[i])
		{
			no_ans = false;
		}
	}

	if (no_ans)
	{
		cout << "No" << endl;
		return 0;
	}

	cout << "Yes" << endl;
	for (int i = 1; i <= n; i++)
	{
		if (a[i] > b[i])
		{
			cout << INF << ' ';
		}
		else
		{
			cout << 1 << ' ';
		}
	}
	cout << endl;
	return 0;
}