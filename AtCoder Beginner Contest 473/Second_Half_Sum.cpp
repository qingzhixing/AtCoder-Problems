#include <iostream>
using namespace std;

const static int MAX_N = 110;
int n;

int main()
{
	cin >> n;
	int sum = 0;
	for (int i = 1; i <= n; i++)
	{
		int num;
		cin >> num;
		if (i > n / 2)
		{
			sum += num;
		}
	}
	cout << sum << endl;
	return 0;
}