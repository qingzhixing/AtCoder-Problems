#include <iostream>
using namespace std;

static const int MAX_N = 110;

int n;
bool exist[MAX_N];

int main()
{
	cin >> n;
	while (n--)
	{
		int num;
		cin >> num;
		exist[num] ^= 1;
	}
	int sum = 0;
	for (int i = 1; i <= MAX_N; i++)
	{
		if (exist[i])
		{
			sum += i;
		}
	}
	cout << sum << endl;
	return 0;
}