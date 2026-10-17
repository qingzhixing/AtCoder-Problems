#include <iostream>
#include <string>
#include <vector>
using namespace std;

const int MAX_N = 110;
const int MAX_M = 10;

bool match(const string &a, const string &b)
{
	if (a.length() != b.length())
	{
		return false;
	}
	int unmatched = 0;
	for (int i = 0; i < a.length(); i++)
	{
		if (a[i] != b[i])
		{
			unmatched++;
		}
	}
	return unmatched == 1;
}

int main()
{
	int n, m;
	cin >> n >> m;
	vector<string> names(n);
	vector<int> received(n);

	for (int i = 0; i < n; i++)
	{
		cin >> names[i];
	}

	for (int vote_id = 0; vote_id < m; vote_id++)
	{
		string vote;
		cin >> vote;

		bool perfectly_match = false;
		int bad_match = 0;
		int bad_match_id = -1;
		for (int id = 0; id < n; id++)
		{
			const auto &name = names[id];
			if (name == vote)
			{
				received[id]++;
				perfectly_match = true;
				break;
			}
			else if (match(name, vote))
			{
				bad_match++;
				bad_match_id = id;
			}
		}
		if (!perfectly_match && bad_match == 1)
		{
			received[bad_match_id]++;
		}
	}

	for (const auto &vote : received)
	{
		cout << vote << endl;
	}
	return 0;
}