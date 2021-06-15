using System;
using System.Collections.Generic;
using System.Linq;

namespace RunRouter
{
    public class RouteData
    {
    

        public RouteData() 
        {
            this.Nodes = ImportNodes();
        }

        private List<Node> ImportNodes()
        {
            List<Node> nodes = new List<Node>();
            string[] lines = System.IO.File.ReadAllLines(@".\nodes.txt");

            foreach (string line in lines)
            {
                nodes.Add(new Node(new NodeName(line.Split(" ")[0], line.Split(" ")[1], line.Split(" ").Length > 2 ? line.Split(" ")[2] : null )));
                
                // Use a tab to indent each line of the file.
                //Console.WriteLine("\t" + line);
            }

            return nodes;
        }

        public List<Node> Nodes {get;set;}

        public bool AddNode(Node node)
        {
            if(this.Nodes.Any(x=> x.Name == node.Name))
            {
                throw new Exception($"Node: {node.Name}");
            }

            this.Nodes.Add(node);
            return true;
        }


        
    }


    public class NodeName {

        public NodeName(string street1, string street2, string qualifier)
        {
            this.Street1 = street1;
            this.Street2 = street2;
            this.Qualifier = qualifier;
        }

        public string Street1 {get;set;}
        public string Street2 {get;set;}

        public string Qualifier {get;set;}
    }

    public class Node
    {
        public Node(NodeName name) {
            this.Name = name;
        }

        public NodeName Name {get;set;}

        public List<Arc> Arcs {get;set;}


        public override string ToString(){
            return this.Name.Street1 + " " + this.Name.Street2 + " " + this.Name.Qualifier;
        }
    }

    public class Arc
    {

        public Arc(Node node1, Node node2, decimal length){
            this.Node1 = node1;
            this.Node2 = node2;
            this.Length = length;
        }

        public Node Node1 {get;set;}
        public Node Node2 {get;set;}

        public decimal Length {get;set;}

        public bool Traversed {get;set;}

    }
}
