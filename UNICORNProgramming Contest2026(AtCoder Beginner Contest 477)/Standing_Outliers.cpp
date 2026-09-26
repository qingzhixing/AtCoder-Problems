#include <iostream>
#include <vector>
#include <algorithm>
using namespace std;

const int MAX_N = 110;

int n, d;

int main()
{
	cin >> n >> d;
	// x, id
	vector<pair<int, int>> position;
	for (int i = 1; i <= n; i++)
	{
		int x;
		cin >> x;
		position.push_back({x, i});
	}

	sort(position.begin(), position.end());

	// id
	vector<int> result;
	for (int i = 0; i < n; i++)
	{
		bool forward = false, backward = false;
		if (i == 0 || position[i].first - position[i - 1].first >= d)
		{
			forward = true;
		}
		if (i == n - 1 || position[i + 1].first - position[i].first >= d)
		{
			backward = true;
		}
		if (forward && backward)
		{
			result.push_back(position[i].second);
		}
	}

	sort(result.begin(), result.end());

	cout << result.size() << endl;
	for (auto id : result)
	{
		cout << id << ' ';
	}
	cout << endl;
	return 0;
}