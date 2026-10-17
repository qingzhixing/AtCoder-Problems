#include <iostream>
#include <utility>
#include <string>
using namespace std;

int main()
{
	pair<int, int> A, D;
	for (int i = 0; i <= 1; i++)
	{
		string str;
		cin >> str;
		for (int j = 0; j < str.size(); j++)
		{
			if (str[j] == 'A')
			{
				A = {i, j};
			}
			if (str[j] == 'D')
			{
				D = {i, j};
			}
		}
	}

	cout << ((abs(A.first - D.first) + abs(A.second - D.second)) == 1 ? "Yes" : "No") << endl;

	return 0;
}