#include <iostream>
#include <vector>
#include <atcoder/segtree>
#include <climits>
using namespace std;

typedef pair<int, int> Node;

Node op_max(Node a, Node b)
{
	return a.first > b.first ? a : b;
}
Node e_max()
{
	return {-1, -1};
}

Node op_min(Node a, Node b)
{
	return a.first < b.first ? a : b;
}
Node e_min()
{
	return {INT_MAX, -1};
}

const int MAX_N = 2e5 + 10;

int main()
{
	int n, m;
	cin >> n >> m;

	vector<int> p(n);
	for (int i = 0; i < n; i++)
	{
		cin >> p[i];
	}

	// a[i] = {p[i], i}
	// 用于建树
	vector<Node> a(n);
	for (int i = 0; i < n; i++)
	{
		a[i] = {p[i], i};
	}

	// 建立维护最大以及最小值的线段树
	atcoder::segtree<Node, op_max, e_max> seg_max(a);
	atcoder::segtree<Node, op_min, e_min> seg_min(a);

	while (m--)
	{
		int l, r;
		cin >> l >> r;
		l--; // 0 - indexed

		// 查询 [l,r)
		auto [max_val, max_id] = seg_max.prod(l, r);
		auto [min_val, min_id] = seg_min.prod(l, r);

		swap(p[max_id], p[min_id]);

		// 更新线段树
		seg_max.set(min_id, {p[min_id], min_id});
		seg_max.set(max_id, {p[max_id], max_id});
		seg_min.set(min_id, {p[min_id], min_id});
		seg_min.set(max_id, {p[max_id], max_id});
	}

	for (auto x : p)
	{
		cout << x << ' ';
	}
	cout << endl;
	return 0;
}