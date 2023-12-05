import random
from networkx.algorithms import bipartite
import networkx as nx

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
        preferences.append([random.random() for j in range(num_items)])

    print("num_of_agent_in_each_group:",n_each)
    print("num_of_groups:",k)
    print("num__of_items:",num_items)
    # print("groups:",groups)
    # print("preferences:",preferences)
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

    
    for p in range(k):
        print("----------Class",p,"evaluates class",p,"'s bunlde as",utility_list[p])
        for q in range(k):
            bundle_q = []
            for agent in groups_util[q]:
                bundle_q += allocation[agent]
            #print("bundle_q",bundle_q)
            cal = cal_maximum_matching(groups_util[p], bundle_q, preferences)
            print("Class",p,"evaluates class",q,"'s bunlde as",cal)

    return 0

if __name__ == '__main__':
    print(main(10,10,1000))
