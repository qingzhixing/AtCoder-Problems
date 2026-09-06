#include <iostream>
using namespace std;

int main()
{
	int n;
	cin >> n;
	int div_10 = 0;
	int count = 0;
	while (n--)
	{
		int a;
		cin >> a;
		if ((a - 1) / 10 != div_10)
		{
			cout << "No" << endl;
			return 0;
		}
		count++;
		div_10 += count / 10;
		count %= 10;
	}
	cout << "Yes" << endl;
	return 0;
}