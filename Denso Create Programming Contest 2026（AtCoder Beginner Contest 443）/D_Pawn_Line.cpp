#include <iostream>
#include <vector>
using namespace std;

void Solution()
{
	long long result = 0;

	int n;
	cin >> n;
	vector<long long> origin_row(n + 10);
	vector<long long> row(n + 10);
	for (int i = 1; i <= n; i++)
	{
		cin >> origin_row[i];
		row[i] = origin_row[i];
	}
	for (int i = 2; i <= n; i++)
	{
		row[i] = min(row[i], row[i - 1] + 1);
	}
	for (int i = n - 1; i >= 1; i--)
	{
		row[i] = min(row[i], row[i + 1] + 1);
	}
	for (int i = 1; i <= n; i++)
	{
		result += origin_row[i] - row[i];
	}
	cout << result << endl;
}

int main()
{
	int t;
	cin >> t;
	while (t--)
	{
		Solution();
	}
	return 0;
}