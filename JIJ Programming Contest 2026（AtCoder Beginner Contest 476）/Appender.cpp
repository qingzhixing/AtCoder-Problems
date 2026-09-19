#include <iostream>
#include <string>
using namespace std;

int main()
{
	string s;
	cin >> s;
	if (s[s.length() - 1] == 'e')
	{
		cout << s + "r" << endl;
	}
	else
	{
		cout << s + "er" << endl;
	}
	return 0;
}