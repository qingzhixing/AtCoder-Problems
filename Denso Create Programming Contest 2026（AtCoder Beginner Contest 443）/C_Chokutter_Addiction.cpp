#include <iostream>
using namespace std;

int main()
{
	int n, t;
	int current_tick = 0;
	int opened_tick = 0;
	cin >> n >> t;
	for (int i = 1; i <= n; i++)
	{
		int appear_tick;
		cin >> appear_tick;
		if (appear_tick < current_tick)
			continue;
		if (appear_tick >= current_tick)
		{
			opened_tick += appear_tick - current_tick;
			current_tick = appear_tick + 100;
		}
	}
	// 判断下班
	if (t >= current_tick)
	{
		opened_tick += t - current_tick;
	}

	cout << opened_tick << endl;
	return 0;
}