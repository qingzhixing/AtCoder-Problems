#include <iostream>
#include <string>
using namespace std;

int main()
{
	int n;
	string s;
	cin >> n >> s;
	int max_R = 0;
	int cnt = 0;
	for (const auto &ch : s)
	{
		if (ch == 'R')
		{
			cnt++;
			max_R = max(max_R, cnt);
		}
		else
		{
			cnt = 0;
		}
	}
	cout << max_R << endl;
	return 0;
}