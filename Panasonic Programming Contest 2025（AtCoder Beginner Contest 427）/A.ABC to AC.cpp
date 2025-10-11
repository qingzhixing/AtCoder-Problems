#include <iostream>
#include <string>
using namespace std;

int main()
{
	string str;
	cin >> str;
	int len = str.length();
	int target_index = (len + 1) / 2 - 1;
	for (int index = 0; index < len; index++)
	{
		if (index == target_index)
			continue;
		cout << str[index];
	}
	return 0;
}