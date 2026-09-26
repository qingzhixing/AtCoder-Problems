#include <iostream>
#include <vector>
#include <string>
using namespace std;

const int MAX_N = 4e5 + 10;

vector<int> kmp_search_all(const string &s, const string &p)
{
	int n = s.size(), m = p.size();
	if (m == 0)
		return {};

	// 计算前缀函数 pi
	vector<int> pi(m);
	for (int i = 1, j = 0; i < m; i++)
	{
		while (j > 0 && p[i] != p[j])
			j = pi[j - 1];
		if (p[i] == p[j])
			j++;
		pi[i] = j;
	}

	// 匹配过程
	vector<int> ans;
	for (int i = 0, j = 0; i < n; i++)
	{
		while (j > 0 && s[i] != p[j])
			j = pi[j - 1];
		if (s[i] == p[j])
			j++;
		if (j == m)
		{
			ans.push_back(i - m + 1); // 记录起始位置（0-indexed）
			j = pi[j - 1];			  // 继续找下一个匹配，允许重叠
		}
	}
	return ans;
}

int main()
{
	int q;
	cin >> q;
	string s, t;
	cin >> s >> t;
	auto match = kmp_search_all(s, t);

	while (q--)
	{
		int l, r;
		cin >> l >> r;
		// to 0-index
		l--;
		r--;
		auto result = lower_bound(match.begin(), match.end(), l);
		if (result == match.end())
		{
			cout << "No" << endl;
			continue;
		}
		if ((*result) + t.size() - 1 > r)
		{
			cout << "No" << endl;
			continue;
		}
		cout << "Yes" << endl;
	}
	return 0;
}