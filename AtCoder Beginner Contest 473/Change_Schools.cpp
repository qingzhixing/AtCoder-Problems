#include <iostream>
using namespace std;

const int MAX_N = 2e5 + 10;

int student_cnt, class_cnt;
int population[MAX_N];

int main()
{
	cin >> student_cnt >> class_cnt;
	for (int i = 1; i <= student_cnt; i++)
	{
		int class_id;
		cin >> class_id;
		population[class_id]++;
	}
	int first_population = 0;
	int first_amount = 0;
	int second_amount = 0;
	for (int i = 1; i <= class_cnt; i++)
	{
		// printf("class %d = %d\n", i, population[i]);
		// 第一大的更新了
		if (population[i] > first_population)
		{
			// 只差 1 则能延顺
			if (population[i] - first_population == 1)
			{
				second_amount = first_amount;
			}
			else
			{
				second_amount = 0;
			}
			first_population = population[i];
			first_amount = 1;
			continue;
		}
		if (population[i] == first_population)
		{
			first_amount++;
			continue;
		}
		if (population[i] == first_population - 1)
		{
			second_amount++;
			continue;
		}
	}
	cout << first_amount + second_amount << endl;
	return 0;
}