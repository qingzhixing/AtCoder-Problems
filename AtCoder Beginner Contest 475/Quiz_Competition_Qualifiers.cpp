#include <iostream>
#include <string>
#include <vector>
using namespace std;

// 一共 NK 个节点
const int MAX_N = 6e6 + 10;

// Trie 树节点
// node[1] 为根
int nxt[MAX_N][2];
int cnt[MAX_N];
int node_cnt = 1;

// val = 1 表示插入，val = -1 表示删除
void insert(const vector<char> &s, int val)
{
	int current_idx = 1;
	cnt[current_idx] += val;
	for (auto num : s)
	{
		// 目标节点不存在, 开辟节点
		if (nxt[current_idx][num] == 0)
		{
			node_cnt++;
			nxt[current_idx][num] = node_cnt;
		}
		current_idx = nxt[current_idx][num];
		cnt[current_idx] += val;
	}
}

int main()
{
	int N, M, K;
	cin >> N >> M >> K;

	string T;
	cin >> T;

	// C[i][j] 表示 i 的第 j 题是否正确
	vector<vector<char>> C(N + 10, vector<char>(K, false));
	for (int i = 1; i <= N; i++)
	{
		string s;
		cin >> s;
		for (int j = 0; j < K; j++)
		{
			C[i][j] = (s[j] == T[j]) ? 1 : 0;
		}
		insert(C[i], 1);
	}

	int Q;
	cin >> Q;
	while (Q--)
	{
		int i, j;
		cin >> i >> j;
		// 第二维从 0 开始
		j--;

		// 删除
		insert(C[i], -1);

		// 翻转
		C[i][j] ^= 1;

		// 重新更新
		insert(C[i], 1);

		// 从树根开始走
		int current_idx = 1;

		// 已通过的人
		int qualifier = 0;

		// 第 i 个人的结果已确定
		bool determined = false;

		// 移动 K 次
		for (int j = 0; j < K; j++)
		{
			// 当前题目正确的人
			int correct = 0;
			if (nxt[current_idx][1] != 0)
			{
				correct = cnt[nxt[current_idx][1]];
			}

			// 我们要走的子树的编号
			int next_direction = -1;
			if (qualifier + correct <= M)
			{
				// 答对晋级，答错存活
				next_direction = 0;
				qualifier += correct;
			}
			else
			{
				// 答对存活，答错淘汰
				next_direction = 1;
			}

			// 第 i 个人的路径与当前路径产生分歧
			// 可以判断是否晋级
			if (C[i][j] != next_direction)
			{
				determined = true;
				if (C[i][j] == 1)
				{
					cout << "Yes" << endl;
					break;
				}
				else
				{
					cout << "No" << endl;
					break;
				}
			}

			current_idx = nxt[current_idx][next_direction];
		}

		// 走完了还没结果直接被淘汰
		if (!determined)
		{
			cout << "No" << endl;
		}
	}
	return 0;
}