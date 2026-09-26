#include <iostream>
using namespace std;

int main()
{
	char ch;
	cin >> ch;
	switch (ch)
	{
	case 'Y':
	{
		cout << 'R' << endl;
		break;
	}
	case 'R':
	{
		cout << 'B' << endl;
		break;
	}
	case 'B':
	{
		cout << 'Y' << endl;
		break;
	}
	}
	return 0;
}