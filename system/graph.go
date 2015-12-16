package system

import "sort"

type graph struct {
	edges [][]int
}

func newGraph(nVertices int) graph {
	g := graph{
		edges: make([][]int, nVertices),
	}

	for i := 0; i < nVertices; i++ {
		g.edges[i] = make([]int, 0, 3)
	}

	return g
}

func (g graph) AddEdge(token int, e int) {
	edges := g.edges[token]

	ip := sort.Search(len(edges), func(i int) bool { return edges[i] > e })
	edges = append(edges, -1)
	copy(edges[ip+1:], edges[ip:])
	edges[ip] = e

	g.edges[token] = edges
}

// Return edges going out of the given vertex.
func (g graph) Edges(vertex int) []int {
	return g.edges[vertex]
}

func (g graph) NVertices() int {
	return len(g.edges)
}

func dependenciesToGraph(dependencies DependencySet) graph {
	depGraph := newGraph(len(dependencies) + 1)

	for dependency := range dependencies {
		depGraph.AddEdge(int(dependency.Head), int(dependency.Dependent))
	}

	return depGraph
}

func dfs(g graph, start int, f func(int)) {
	// Note: the graph should be DAG.
	edges := g.Edges(start)

	var idx int
	for idx = 0; idx < len(edges); idx++ {
		if edges[idx] > start {
			break
		}

		dfs(g, edges[idx], f)
	}

	f(start)

	for ; idx < len(edges); idx++ {
		dfs(g, edges[idx], f)
	}
}
