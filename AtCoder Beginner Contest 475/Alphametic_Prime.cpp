#include <iostream>
#include <string>
#include <vector>
using namespace std;

const int MAX_M = 1e7 + 10;
bool filtered[MAX_M];
vector<int> primes;

// 线性筛
void init_primes()
{
	filtered[0] = true;
	filtered[1] = true;
	for (int i = 2; i < MAX_M; i++)
	{
		if (!filtered[i])
		{
			primes.push_back(i);
		}
		for (auto prime : primes)
		{
			if (i * prime >= MAX_M)
			{
				break;
			}
			filtered[i * prime] = true;
			if (i % prime == 0)
			{
				break;
			}
		}
	}
}

bool is_prime(int n)
{
	return !filtered[n];
}

string s;

vector<int> ans;

// 找到质数并退出程序
void dfs(int idx)
{
	// 找完最后一位了
	if (idx == s.length())
	{
		// 包含前导零
		if (ans.size() > 1 && ans[0] == 0)
		{
			return;
		}

		int number = 0;
		for (auto digit : ans)
		{
			number = number * 10 + digit;
		}

		// 不是质数
		if (!is_prime(number))
		{
			return;
		}

		cout << number << endl;
		exit(0);
	}

	// 分配当前位
	// 如果之前出现过同样的字符则直接分配

	int pre_idx = -1;
	for (int i = 0; i < idx; i++)
	{
		if (s[i] == s[idx])
		{
			pre_idx = i;
			break;
		}
	}

	if (pre_idx != -1)
	{
		ans.push_back(ans[pre_idx]);
		dfs(idx + 1);
		ans.pop_back();
		return;
	}

	// 否则新开一个数字
	// 标记当前数字是否被占用

	bool occupied[20] = {};

	for (auto digit : ans)
	{
		occupied[digit] = true;
	}

	for (int i = 0; i <= 9; i++)
	{
		if (!occupied[i])
		{
			ans.push_back(i);
			dfs(idx + 1);
			ans.pop_back();
		}
	}
}

int main()
{
	init_primes();
	cin >> s;

	dfs(0);
	// 没退出说明没找到
	cout << -1 << endl;
	return 0;
}