#include <iostream>
#include <algorithm>
using namespace std;

const int MAX_N = 2e5 + 10;

int n, k;
int a[MAX_N];
int sorted[MAX_N];

int main()
{
	cin >> n >> k;
	for (int i = 1; i <= n; i++)
	{
		cin >> a[i];
		sorted[i] = a[i];
	}

	sort(sorted + 1, sorted + 1 + n);

	int l = 1, r = n;

	while (l <= n && a[l] == sorted[l])
	{
		l++;
	}

	while (r >= 1 && a[r] == sorted[r])
	{
		r--;
	}

	int unsorted_len = max(0, r - l + 1);

	if (unsorted_len <= k)
	{
		cout << "Yes" << endl;
	}
	else
	{
		cout << "No" << endl;
	}

	return 0;
}