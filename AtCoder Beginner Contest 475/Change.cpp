#include <iostream>
using namespace std;

const int MAX_N = 1010;

int n;
int coin_1, coin_10, coin_100;

int main()
{
	cin >> n;
	while (n--)
	{
		int num;
		cin >> num;
		int remain = 1000 - (num % 1000);
		coin_1 += remain % 10;
		remain /= 10;
		coin_10 += remain % 10;
		remain /= 10;
		coin_100 += remain % 10;
	}
	printf("%d %d %d\n", coin_1, coin_10, coin_100);
	return 0;
}