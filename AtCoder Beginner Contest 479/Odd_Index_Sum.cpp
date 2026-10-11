#include <iostream>
using namespace std;

const int MAX_N = 0;
const int MAX_M = 0;

int main()
{
	int n;
	cin >> n;
	int result = 0;
	for (int i = 1; i <= n; i++)
	{
		int a;
		cin >> a;
		if (i & 1)
		{
			result += a;
		}
	}
	cout << result << endl;
	return 0;
}