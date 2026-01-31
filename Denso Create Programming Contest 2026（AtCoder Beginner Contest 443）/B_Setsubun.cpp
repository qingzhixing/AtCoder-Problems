#include <iostream>
using namespace std;

int main()
{
	int n, k;
	cin >> n >> k;
	long long year = 0;
	long long sum = 0;
	while (sum < k)
	{
		sum += n + year;
		year++;
	}
	cout << (year - 1) << endl;
	return 0;
}