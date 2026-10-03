#include <iostream>
using namespace std;

const int MAX_N = 110;

int n, v;
int w[MAX_N];

int main()
{
	cin >> n >> v;
	for (int i = 1; i <= n; i++)
	{
		cin >> w[i];
	}

	int result = 0;

	for (int i = 1; i <= n; i++)
	{
		for (int j = 1; j <= n; j++)
		{
			for (int k = 1; k <= n; k++)
			{
				if (i == j || j == k || i == k)
				{
					continue;
				}
				if (i + j + k > v)
				{
					continue;
				}
				result = max(result, w[i] + w[j] + w[k]);
			}
		}
	}

	cout << result << endl;
	return 0;
}