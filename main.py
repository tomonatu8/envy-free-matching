import random
import networkx as nx
import matplotlib.pyplot as plt
import numpy as np
from scipy.stats import truncnorm

def round_robin_allocation_by_group(num_items, groups, preferences):
    allocation = {agent: [] for group in groups for agent in group}
    items = [i for i in range(num_items)]
    available_items = set(items)

    # グループ間でラウンドロビンを行う
    while available_items:
        for group in groups:
            if group == []:
                break
            # グループ内で最も高い評価値を持つアイテムを選ぶエージェントを見つける
            best_choice = None
            best_value = -1

            for agent in group:
                if available_items:
                    best_item_for_agent = max(available_items, key=lambda x: preferences[agent][x])
                    if preferences[agent][best_item_for_agent] > best_value:
                        best_choice = (agent, best_item_for_agent)
                        best_value = preferences[agent][best_item_for_agent]

            # print(best_choice)
            if best_choice != None:
                # アイテムを選択し、利用可能なアイテムから削除
                if best_choice:
                    agent, item = best_choice
                    allocation[agent].append(item)
                    available_items.remove(item)
                    group.remove(agent)
        if all(element == [] for row in groups for element in row):
            break

    return allocation

# 使用例
# num_items = 5
# groups = [[0, 1], [2, 3], [4, 5]]
# num_agents = 6
# preferences = []
# for i in range(num_agents):
#     preferences.append([j for j in range(num_items)])

# print(preferences)
# allocation = round_robin_allocation_by_group(num_items, groups, preferences)
# print(allocation)
# for i in allocation:
#     print("Agent",i,"s' items: ", allocation[i])


def cal_maximum_matching(left_list, right_list, preferences):
    B = nx.Graph()
    B.add_nodes_from(left_list, bipartite=0)
    edges = []
    for i in left_list:
        for j in right_list:
            edges.append((i,str(j),preferences[i][j]))
    #print(edges)
    #edges = [(1, 2, 6), (1, 3, 2), (2, 3, 1), (2, 4, 7), (3, 5, 9), (4, 5, 3)]
    B.add_weighted_edges_from(edges)
    max_edges = nx.max_weight_matching(B)
    #print(max_edges)
    sum = 0
    for edge in max_edges:
        #print(edge)
        if type(edge[0]) is int:
            sum += preferences[edge[0]][int(edge[1])]
        else:
            sum += preferences[edge[1]][int(edge[0])]
    return sum

def main(n_each, k, num_items):
    # groups = [[0, 1], [2, 3], [4, 5]]
    groups = []
    for i in range(k):
        groups.append([i*n_each + j for j in range(n_each)]) 
    num_agents = n_each * k
    preferences = []
    for i in range(num_agents):
        # preferences.append(generate_uniform_random_list(num_items))
        preferences.append(generate_truncnorm_list(num_items))

    # print("groups:",groups)
    # print("preferences:",preferences)

    max_w_mat = cal_maximum_matching(range(k*n_each), range(num_items), preferences)
    # print("全体の最大重みマッチングの値は",max_w_mat)
    # print("そのうち平均して",max_w_mat/k)
    # 最大重みマッチングがどれくらいEFやEF1になるか調べる

    allocation = round_robin_allocation_by_group(num_items, groups, preferences)

    # print("allocation:",allocation)

    groups_util = []
    for i in range(k):
        groups_util.append([i*n_each + j for j in range(n_each)]) 
    utility_list = []
    for i in range(k):
        utility = 0
        for agent in groups_util[i]:
            if allocation[agent] != []:
                utility += preferences[agent][allocation[agent][0]]
        utility_list.append(utility)

    utility_list_other = []
    
    for p in range(k):
    #     print("----------Class",p,"evaluates class",p,"'s bunlde as",utility_list[p])
    #     print("----------Class",p,"evaluates whole set of item as", cal_maximum_matching(groups_util[p], range(num_items), preferences))

        utility_list_other_each = []
        for q in range(k):
            bundle_q = []
            for agent in groups_util[q]:
                bundle_q += allocation[agent]
            #print("bundle_q",bundle_q)
            cal = cal_maximum_matching(groups_util[p], bundle_q, preferences)
            # print("Class",p,"evaluates class",q,"'s bunlde as",cal)
            utility_list_other_each.append(cal)
        utility_list_other.append(utility_list_other_each)

    return utility_list, utility_list_other, max_w_mat/k

def generate_uniform_random_list(num_items):
    return [random.random() for j in range(num_items)]

def generate_truncnorm_list(num_items):

    lower_clip = 0.
    upper_clip = 1.
    mu = 0.5
    sd = 0.3

    p = truncnorm.rvs((lower_clip - mu) / sd, (upper_clip - mu) / sd, mu, sd, size=num_items)
    return list(p)

if __name__ == '__main__':
    # lower_clip = 0.
    # upper_clip = 1.
    # mu = 0.5
    # sd = 0.3

    # plt.hist(truncnorm.rvs((lower_clip - mu) / sd, (upper_clip - mu) / sd, mu, sd, size=1000), alpha=0.5)
    # plt.show()

    # print(main(10,10,1000))
    n_each = 100
    k = 2
    num_items = 250

    print("num_of_agent_in_each_group:",n_each)
    print("num_of_groups:",k)
    print("num__of_items:",num_items)

    all_utility_list = []
    all_utility_list_other = []

    all_max_weight_match_list = []

    for i in range(100):
        print(i)
        utility_list, utility_list_other, max_weight_match = main(n_each, k, num_items)
        # print(utility_list)
        # print(utility_list_other)
        all_utility_list += utility_list
        all_max_weight_match_list.append(max_weight_match)
        for j in range(len(utility_list_other)):
            for l in range(100):
                if l != j:
                    all_utility_list_other.append(utility_list_other[j][l])
                    break

    print("mean(all_utility_list)",np.mean(all_utility_list))
    print("mean(all_utility_list_other)",np.mean(all_utility_list_other))
    print("mean(all_utility_list) - mean(all_utility_list_other)",np.mean(all_utility_list) - np.mean(all_utility_list_other))

    # ヒストグラムの描画
    plt.hist(all_utility_list, bins=100, alpha=0.5, label='Utilities')
    plt.hist(all_utility_list_other, bins=100, alpha=0.5, label='Other Value')
    plt.hist(all_max_weight_match_list, bins=100, alpha=0.5, label='Max weight matching/k')

    # 凡例の追加
    plt.legend()

    # タイトル
    plt.title('('+str(n_each) + ',' + str(k) + ',' + str(num_items)+')')

    # グラフの表示
    plt.show()
