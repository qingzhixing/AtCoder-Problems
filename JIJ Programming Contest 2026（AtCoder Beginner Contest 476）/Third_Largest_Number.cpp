#include <iostream>
using namespace std;

const int MAX_N = 5e5 + 10;
const int INF = 1e9 + 10;

int n;
int m1, m2, m3;

int main()
{
	cin >> n;
	m1 = m2 = m3 = -INF;

	for (int i = 1; i <= n; i++)
	{
		int a;
		cin >> a;

		if (a > m1)
		{
			m3 = m2;
			m2 = m1;
			m1 = a;
		}
		else if (a > m2)
		{
			m3 = m2;
			m2 = a;
		}
		else if (a > m3)
		{
			m3 = a;
		}

		if (i >= 3)
		{
			cout << m3 << endl;
		}
	}
	return 0;
}