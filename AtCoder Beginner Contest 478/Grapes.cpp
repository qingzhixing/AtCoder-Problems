#include <iostream>
using namespace std;

int main()
{
	int n, m;
	cin >> n >> m;
	for (int i = 1; i <= n; i++)
	{
		if (i <= m % n)
		{
			cout << m / n + 1 << endl;
		}
		else
		{
			cout << m / n << endl;
		}
	}
	return 0;
}