from src.fsm import *
import graphviz

def make_nodes(graph,nodes):
    for node in nodes:
        if type(node) is list:
            graph.node(f'[{",".join(map(str,node))}]')
        else:
            graph.node(str(node))

def make_transitions(graph,transitions):
    for start, symbol, end in transitions:
        if symbol == "epsilon":
            symbol = "ε"
        
        if type(start) is list:
            start=f'[{",".join(map(str,start))}]'

        if type(end) is list:
            end=f'[{",".join(map(str,end))}]'

        graph.edge(str(start),str(end),label=str(symbol))

def make_start(graph,start_node):
    graph.node("",shape="none")
    if type(start_node) is list:
        graph.edge("",f'[{",".join(map(str,start_node))}]')
    else:
        graph.edge("",str(start_node))

def make_finals(graph,final_states):
    for state in final_states:
        if type(state) is list:
            graph.node(f'[{",".join(map(str,state))}]',shape="doublecircle")
        else:
            graph.node(str(state),shape="doublecircle")

#Main Function
#filename - the name of your output file - type str
#fsm - input your Fsm object
#cleanup - bool(True or False) if you want to delete the files generated from making your visualization
def make_visual(fsm,filename="output",cleanup=True):
    graph = graphviz.Digraph(filename, comment='NFA',engine='dot',graph_attr={'rankdir':'LR'})

    make_nodes(graph,fsm.states)
    make_transitions(graph,fsm.transitions)
    make_start(graph,fsm.start)
    make_finals(graph,fsm.final)
    graph.render(directory='visual_output', view=True, quiet=True, quiet_view=True, cleanup=cleanup)
#Call make_visual(filename,fsm,cleanup) to view your NFA
