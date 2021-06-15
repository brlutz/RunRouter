using System;

namespace RunRouter
{
    class Program
    {
        static void Main(string[] args)
        {
            RouteData data = new RouteData();

            foreach(Node n in data.Nodes)
            {
                Console.WriteLine(n.ToString());
            }
        }
    }
}
