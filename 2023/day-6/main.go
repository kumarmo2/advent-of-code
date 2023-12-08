package main

import (
	"fmt"
	"os"
	"strings"
)

type Node struct {
	Name  string
	Left  *Node
	Right *Node
}

func main() {
	content, err := os.ReadFile("./input-prod.txt")
	if err != nil {
		panic(err)
	}
	input := string(content)
	// fmt.Println(input)
	fmt.Println(part1(input))

}

func part1(input string) uint64 {
	lines := strings.Split(input, "\n")
	directions := lines[0]
	nodeLines := lines[2:]
	mapOfNodes := make(map[string]*Node)

	for _, nodeLine := range nodeLines {
		if nodeLine == "" {
			break
		}

		items := strings.Split(nodeLine, "=")
		nodeName := strings.Trim(items[0], " ")
		node, ok := mapOfNodes[nodeName]
		if !ok {
			node = &Node{Name: nodeName}
			mapOfNodes[nodeName] = node
		}
		childNodes := strings.Split(strings.ReplaceAll(strings.ReplaceAll(strings.Trim(items[1], " "), "(", ""), ")", ""), ",") // strings.Trim(items[1], "")
		left := strings.Trim(childNodes[0], " ")
		right := strings.Trim(childNodes[1], " ")
		// fmt.Printf("node: %v, left: %v, right: %v\n", nodeName, left, right)
		leftNode, ok := mapOfNodes[left]
		if !ok {
			leftNode = &Node{Name: left}
			mapOfNodes[left] = leftNode
		}
		rightNode, ok := mapOfNodes[right]
		if !ok {
			rightNode = &Node{Name: right}
			mapOfNodes[right] = rightNode
		}
		node.Left = leftNode
		node.Right = rightNode
		// json, _ := json.Marshal(node)

		// fmt.Println("actual node: %v", string(json))
	}
	// panic("unimplemented")

	// nodes, _ := json.Marshal(mapOfNodes)
	// fmt.Println(string(nodes))
	// for key, node := range mapOfNodes {
	// 	fmt.Printf("node: %v, left: %v, right: %v\n", key, node.Left.Name, node.Right.Name)
	//
	// }
	fmt.Println("===========")

	currNode, _ := mapOfNodes["AAA"]
	n := len(directions)
	i := 0
	var steps uint64 = 0

	for {

		if currNode.Name == "ZZZ" {
			return steps
		}
		if i == n {
			i = 0
		}
		direction := directions[i]
		if direction == 'L' {
			currNode = currNode.Left
		} else {
			currNode = currNode.Right
		}
		i++
		steps++
		// direction := strings.ch
	}

	return 0
}