#include <iostream>
#include <string>
using namespace std;

int n;
string a, b;

int main()
{
	cin >> n;
	cin >> a >> b;
	for (auto i = 0; i < n; i++)
	{
		if (b[i] == '*')
		{
			continue;
		}
		if (a[i] != b[i])
		{
			cout << "No" << endl;
			return 0;
		}
	}
	cout << "Yes" << endl;
	return 0;
}